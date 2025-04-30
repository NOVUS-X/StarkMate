use actix_web::web;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::models::*;

const ELO_RANGE_INCREMENT_PER_MINUTE: u32 = 50;
const DEFAULT_MAX_ELO_DIFF: u32 = 200;
const DEFAULT_ESTIMATED_WAIT_TIME: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct MatchmakingService {
    queue: Arc<Mutex<MatchmakingQueue>>,
    active_matches: Arc<Mutex<HashMap<Uuid, Match>>>,
}

impl MatchmakingService {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(MatchmakingQueue::new())),
            active_matches: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn join_queue(&self, request: MatchRequest) -> MatchmakingResponse {
        let mut queue = self.queue.lock().unwrap();
        let request_id = request.id;

        match request.match_type {
            MatchType::Rated => {
                // Try to find a match first
                if let Some(match_result) = self.find_rated_match(&request, &mut queue) {
                    return match_result;
                }
                // Otherwise add to queue
                queue.rated_queue.push(request);
            }
            MatchType::Casual => {
                // Try to find a match first
                if let Some(match_result) = self.find_casual_match(&request, &mut queue) {
                    return match_result;
                }
                // Otherwise add to queue
                queue.casual_queue.push(request);
            }
            MatchType::Private => {
                // For private matches, we store the invite
                if let Some(invite_address) = &request.invite_address {
                    queue.private_invites.insert(invite_address.clone(), request);
                    return MatchmakingResponse {
                        status: "Waiting for invited player".to_string(),
                        match_id: None,
                        request_id,
                    };
                } else {
                    return MatchmakingResponse {
                        status: "Invalid private match request: missing invite address".to_string(),
                        match_id: None,
                        request_id,
                    };
                }
            }
        }

        MatchmakingResponse {
            status: "Added to queue".to_string(),
            match_id: None,
            request_id,
        }
    }

    pub fn check_private_invite(&self, wallet_address: &str) -> Option<MatchRequest> {
        let queue = self.queue.lock().unwrap();
        queue.private_invites.get(wallet_address).cloned()
    }

    pub fn accept_private_invite(
        &self,
        inviter_request_id: Uuid,
        accepting_player: Player,
    ) -> Option<MatchmakingResponse> {
        let mut queue = self.queue.lock().unwrap();
        
        // Find the invite by request ID
        let invite_entry = queue.private_invites.iter()
            .find(|(_, req)| req.id == inviter_request_id);
        
        if let Some((invite_address, invite_request)) = invite_entry.cloned() {
            // Remove the invite
            queue.private_invites.remove(&invite_address);
            
            // Create the match
            let match_id = Uuid::new_v4();
            let new_match = Match {
                id: match_id,
                player1: invite_request.player,
                player2: accepting_player,
                match_type: MatchType::Private,
                created_at: Instant::now(),
            };
            
            // Store the match
            let mut active_matches = self.active_matches.lock().unwrap();
            active_matches.insert(match_id, new_match);
            
            Some(MatchmakingResponse {
                status: "Match created".to_string(),
                match_id: Some(match_id),
                request_id: inviter_request_id,
            })
        } else {
            None
        }
    }

    pub fn cancel_request(&self, request_id: Uuid) -> bool {
        let mut queue = self.queue.lock().unwrap();
        
        // Check rated queue
        if let Some(index) = queue.rated_queue.iter().position(|req| req.id == request_id) {
            queue.rated_queue.remove(index);
            return true;
        }
        
        // Check casual queue
        if let Some(index) = queue.casual_queue.iter().position(|req| req.id == request_id) {
            queue.casual_queue.remove(index);
            return true;
        }
        
        // Check private invites
        let invite_key = queue.private_invites.iter()
            .find(|(_, req)| req.id == request_id)
            .map(|(key, _)| key.clone());
        
        if let Some(key) = invite_key {
            queue.private_invites.remove(&key);
            return true;
        }
        
        false
    }

    pub fn get_queue_status(&self, request_id: Uuid) -> Option<QueueStatus> {
        let queue = self.queue.lock().unwrap();
        
        // Check rated queue
        if let Some(index) = queue.rated_queue.iter().position(|req| req.id == request_id) {
            return Some(QueueStatus {
                request_id,
                position: index + 1,
                estimated_wait_time: self.estimate_wait_time(index, &MatchType::Rated),
                match_type: MatchType::Rated,
            });
        }
        
        // Check casual queue
        if let Some(index) = queue.casual_queue.iter().position(|req| req.id == request_id) {
            return Some(QueueStatus {
                request_id,
                position: index + 1,
                estimated_wait_time: self.estimate_wait_time(index, &MatchType::Casual),
                match_type: MatchType::Casual,
            });
        }
        
        // Check private invites
        for (_, req) in queue.private_invites.iter() {
            if req.id == request_id {
                return Some(QueueStatus {
                    request_id,
                    position: 1,
                    estimated_wait_time: DEFAULT_ESTIMATED_WAIT_TIME,
                    match_type: MatchType::Private,
                });
            }
        }
        
        None
    }

    fn find_rated_match(
        &self,
        request: &MatchRequest,
        queue: &mut MatchmakingQueue,
    ) -> Option<MatchmakingResponse> {
        let player_elo = request.player.elo;
        let max_elo_diff = request.max_elo_diff.unwrap_or(DEFAULT_MAX_ELO_DIFF);
        
        // Find a suitable opponent based on ELO
        let opponent_index = queue.rated_queue.iter().position(|req| {
            let elo_diff = if req.player.elo > player_elo {
                req.player.elo - player_elo
            } else {
                player_elo - req.player.elo
            };
            
            // Check if within ELO range
            elo_diff <= max_elo_diff
        });
        
        if let Some(index) = opponent_index {
            let opponent_request = queue.rated_queue.remove(index);
            let match_id = Uuid::new_v4();
            
            // Create the match
            let new_match = Match {
                id: match_id,
                player1: opponent_request.player,
                player2: request.player.clone(),
                match_type: MatchType::Rated,
                created_at: Instant::now(),
            };
            
            // Store the match
            let mut active_matches = self.active_matches.lock().unwrap();
            active_matches.insert(match_id, new_match);
            
            Some(MatchmakingResponse {
                status: "Match found".to_string(),
                match_id: Some(match_id),
                request_id: request.id,
            })
        } else {
            None
        }
    }

    fn find_casual_match(
        &self,
        request: &MatchRequest,
        queue: &mut MatchmakingQueue,
    ) -> Option<MatchmakingResponse> {
        // For casual matches, just match with the first player in queue
        if !queue.casual_queue.is_empty() {
            let opponent_request = queue.casual_queue.remove(0);
            let match_id = Uuid::new_v4();
            
            // Create the match
            let new_match = Match {
                id: match_id,
                player1: opponent_request.player,
                player2: request.player.clone(),
                match_type: MatchType::Casual,
                created_at: Instant::now(),
            };
            
            // Store the match
            let mut active_matches = self.active_matches.lock().unwrap();
            active_matches.insert(match_id, new_match);
            
            Some(MatchmakingResponse {
                status: "Match found".to_string(),
                match_id: Some(match_id),
                request_id: request.id,
            })
        } else {
            None
        }
    }

    fn estimate_wait_time(&self, position: usize, match_type: &MatchType) -> Duration {
        match match_type {
            MatchType::Rated => {
                // For rated matches, wait time increases with position
                Duration::from_secs((30 + position as u64 * 15).min(300))
            }
            MatchType::Casual => {
                // For casual matches, usually faster
                Duration::from_secs((15 + position as u64 * 10).min(180))
            }
            MatchType::Private => {
                // For private matches, depends on when the invited player accepts
                DEFAULT_ESTIMATED_WAIT_TIME
            }
        }
    }

    // Periodically run this to expand ELO range for players waiting too long
    pub fn expand_elo_ranges(&self) {
        let mut queue = self.queue.lock().unwrap();
        let now = Instant::now();
        
        for request in queue.rated_queue.iter_mut() {
            let wait_time = now.duration_since(request.player.join_time);
            let minutes_waiting = wait_time.as_secs() / 60;
            
            if minutes_waiting > 0 {
                // Expand ELO range based on wait time
                let additional_range = minutes_waiting as u32 * ELO_RANGE_INCREMENT_PER_MINUTE;
                request.max_elo_diff = Some(
                    request.max_elo_diff.unwrap_or(DEFAULT_MAX_ELO_DIFF) + additional_range
                );
            }
        }
    }

    pub fn get_match(&self, match_id: Uuid) -> Option<Match> {
        let active_matches = self.active_matches.lock().unwrap();
        active_matches.get(&match_id).cloned()
    }
}

// Factory function for dependency injection
pub fn get_matchmaking_service() -> web::Data<MatchmakingService> {
    web::Data::new(MatchmakingService::new())
}
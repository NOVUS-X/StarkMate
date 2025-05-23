use super::{Chess960Generator, Chess960Position};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Chess960PositionsManager {
    positions: HashMap<u16, Chess960Position>,
}

impl Chess960PositionsManager {
    /// Create new positions manager
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }
    
    /// Load all positions into memory
    pub fn load_all_positions(&mut self) {
        let all_positions = Chess960Generator::generate_all_positions();
        
        for position in all_positions {
            self.positions.insert(position.position_number, position);
        }
    }
    
    /// Get position by number
    pub fn get_position(&self, position_number: u16) -> Option<&Chess960Position> {
        self.positions.get(&position_number)
    }
    
    /// Get all positions
    pub fn get_all_positions(&self) -> Vec<&Chess960Position> {
        let mut positions: Vec<&Chess960Position> = self.positions.values().collect();
        positions.sort_by_key(|p| p.position_number);
        positions
    }
    
    /// Export positions to JSON file
    pub fn export_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let positions: Vec<&Chess960Position> = self.get_all_positions();
        let json_data = serde_json::to_string_pretty(&positions)?;
        fs::write(file_path, json_data)?;
        Ok(())
    }
    
    /// Load positions from JSON file
    pub fn load_from_json(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(file_path).exists() {
            let json_data = fs::read_to_string(file_path)?;
            let positions: Vec<Chess960Position> = serde_json::from_str(&json_data)?;
            
            self.positions.clear();
            for position in positions {
                self.positions.insert(position.position_number, position);
            }
        } else {
            // Generate if file doesn't exist
            self.load_all_positions();
            self.export_to_json(file_path)?;
        }
        
        Ok(())
    }
}
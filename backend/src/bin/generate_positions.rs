use std::fs;
use starkmate::chess960::{Chess960Generator, Chess960PositionsManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = Chess960PositionsManager::new();
    manager.load_all_positions();
    
    // Create resources directory if it doesn't exist
    fs::create_dir_all("resources")?;
    
    // Export to JSON
    manager.export_to_json("resources/chess960_positions.json")?;
    
    println!("Generated chess960_positions.json with all 960 positions");
    Ok(())
}
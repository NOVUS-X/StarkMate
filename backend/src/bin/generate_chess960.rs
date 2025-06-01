
use backend::chess960::Chess960Generator;

fn main() {
    println!("Generating Chess960 FEN library...");
    
    let library = Chess960Generator::generate_all_positions();
    println!("Generated {} positions", library.total_positions);
    
    // Ensure resources directory exists
    std::fs::create_dir_all("resources").expect("Failed to create resources directory");
    
    // Save complete library
    let json = serde_json::to_string_pretty(&library).expect("Failed to serialize library");
    std::fs::write("resources/chess960_complete.json", json).expect("Failed to write complete library file");
     
     // Save compact version (just FEN strings)
    let compact: std::collections::HashMap<u16, String> = library.positions.iter()
    .map(|(&id, position)| (id, position.fen.clone()))
    .collect();

     
    let compact_json = serde_json::to_string_pretty(&compact).expect("Failed to serialize compact library");
    std::fs::write("resources/chess960_fens.json", compact_json).expect("Failed to write compact library file");
    println!("Files saved:");
    println!("- resources/chess960_complete.json (complete data)");
    println!("- resources/chess960_fens.json (FEN strings only)");
    
    // Display sample positions
    println!("\nSample positions:");
    for i in 1..=5 {
        if let Some(pos) = library.positions.get(&i) {
            println!("Position {}: {} ({})", i, pos.fen, pos.back_rank);
        }
    }
}

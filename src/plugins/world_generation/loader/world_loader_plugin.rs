use std::fs::{File, self};
use std::io::{BufRead, BufReader};
use bevy::prelude::*;

use crate::plugins::world_generation::level_plugin::{Level, LevelLayer, LevelTile};
use crate::plugins::world_generation::world_plugin::Worlds;



pub struct WorldLoaderPlugin;


impl Plugin for WorldLoaderPlugin  {
    fn build(&self, app: &mut App) {
        warn!("Initialize: WorldLoaderPlugin");
        app.init_resource::<Worlds>().add_systems(PreStartup, init_worlds);
    }
}


fn read_instructions(file_path: &str) -> Vec<(String, usize, usize, String, String)> {
    let mut instructions: Vec<(String, usize, usize, String, String)> = Vec::new();
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open the file: {}", e);
            return instructions;
        }
    };

    let reader = BufReader::new(file);
    for (y_pos, line) in reader.lines().enumerate() {
        if let Ok(data) = line {
            println!("Line {}: {}", y_pos, data);
            let mut x = 0; // Reset x for each new line
            for item in data.split('$') {
                if !item.is_empty() {
                    if let Some((instruction, tile_id)) = extract_instruction_tile_id(item, x, y_pos) {
                        instructions.push((instruction, x, y_pos, tile_id, "".to_string()));
                    }
                    x += 1; // Increment x inside the loop to change for each item split
                }
            }
        }
    }
    instructions
}


fn extract_instruction_tile_id(cell_data: &str, x: usize, y: usize) -> Option<(String, String)> {
    let cleaned_cell_data = cell_data.trim_start_matches('#').trim();

    if let Some(opening_bracket_index) = cleaned_cell_data.find('(') {
        if let Some(closing_bracket_index) = cleaned_cell_data[opening_bracket_index..].find(')') {
            let instruction = &cleaned_cell_data[opening_bracket_index + 1..opening_bracket_index + closing_bracket_index];
            let rest = &cleaned_cell_data[closing_bracket_index + opening_bracket_index + 1..].trim();
            let tile_id = if rest.is_empty() { "".to_string() } else { rest.to_string() };
            println!(
                "Loop1 Extracted: x={}, y={}, instruction={}, tile_id={}",
                x, y, instruction, tile_id
            );
            return Some((instruction.to_string(), tile_id));
        }
    } else if let Some(closing_bracket_index) = cleaned_cell_data.find(')') {
        let instruction = &cleaned_cell_data[..closing_bracket_index];
        let rest = &cleaned_cell_data[closing_bracket_index + 1..].trim();
        let tile_id = if rest.is_empty() { "".to_string() } else { rest.to_string() };
        println!(
            "Loop2 Extracted: x={}, y={}, instruction={}, tile_id={}",
            x, y, instruction, tile_id
        );
        return Some((instruction.to_string(), tile_id));
    }

    error!("Failed to extract: x={}, y={}, cell_data={}", x, y, cell_data);
    None
}



fn init_worlds(worlds: ResMut<Worlds>) {
    let map_path = "assets/maps/";

    if let Ok(entries) = fs::read_dir(map_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    if let Some(level_name) = entry.file_name().to_str() {
                        let mut level = Level::new(level_name.to_string(), LevelLayer::default(), LevelLayer::default(), LevelLayer::default());
                        for i in 0..3 {
                            let file_path = format!("{}/{}/{}/{}.map", map_path, level_name, i, i);
                            warn!("Map found at: {}", file_path);
                            if !std::path::Path::new(&file_path).exists() {
                                continue; // Skip if the file does not exist
                            }
                            let instructions = read_instructions(&file_path);

                            let mut layer = LevelLayer::default();
                            for y1 in 0..16 {
                                for x1 in 0..16 {
                                    // ... (Rest of the tile loading logic remains the same)

                                    for (instruction, _pos, _tile_code, tile_id, _timestamp) in &instructions {
                                        layer.add_tile(LevelTile::new(x1, y1, instruction.clone(), tile_id.clone()));
                                        info!("    pos = {}x{} instruction = {}", x1 ,y1 , instruction.clone().to_string());
                                    }
                                }
                            }

                            match i {
                                0 => level.set_layer_0(layer),
                                1 => level.set_layer_1(layer),
                                2 => level.set_layer_2(layer),
                                _ => (),
                            }
                        }
                        worlds.insert_level(level);
                    }
                }
            }
        }
    }
}

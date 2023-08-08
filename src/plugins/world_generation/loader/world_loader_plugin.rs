use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::prelude::*;



pub struct WorldLoaderPlugin;


impl Plugin for WorldLoaderPlugin  {
    fn build(&self, app: &mut App) {
        warn!("Initialize: WorldLoaderPlugin");
        app.add_systems(PreStartup, init_worlds);
    }
}

fn init_worlds(){
    read_map("test", 16, 16);
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
            //println!("Line {}: {}", y_pos, data);
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
            /*
            println!(
                "Extracted: x={}, y={}, instruction={}, tile_id={}",
                x, y, instruction, tile_id
            );
            */
            return Some((instruction.to_string(), tile_id));
        }
    } else if let Some(closing_bracket_index) = cleaned_cell_data.find(')') {
        let instruction = &cleaned_cell_data[..closing_bracket_index];
        let rest = &cleaned_cell_data[closing_bracket_index + 1..].trim();
        let tile_id = if rest.is_empty() { "".to_string() } else { rest.to_string() };
        /*
        println!(
            "Extracted: x={}, y={}, instruction={}, tile_id={}",
            x, y, instruction, tile_id
        );
        */
        return Some((instruction.to_string(), tile_id));
    }

    error!("Failed to extract: x={}, y={}, cell_data={}", x, y, cell_data);
    None
}


pub fn read_map(world_name: &str, x: usize, y: usize) {
    //let mut unique_hash_map = UniqueHashMap::new(1000000);

    for i in 0..3 {
        let file_path = format!("assets/maps/{}/{}.map", world_name, i);
        let instructions = read_instructions(&file_path);

        if instructions.is_empty() {
            error!("Failed to read instructions for map: {} Level: {}", world_name, i);
            continue;
        }

        warn!("Instructions for {} World {}: ", world_name, i);

        for y1 in 0..y
         {
            for x1 in 0..x {
                let _pos = format!("{}x{}", x1, y1);
                let mut _extract_instruction_tile_id = "".to_string();
                /*
                for (instruction, pos, _tile_code, tile_id, _timestamp) in &instructions {
                    
                    
                    unique_hash_map.insert(
                        world_name.clone().to_string(),
                        pos.clone().to_string(),
                        i.clone().to_string(),
                        instruction.clone().to_string(),
                        tile_id.clone().to_string(),
                    );
                    extract_instruction_tile_id = instruction.clone().to_string();
                }
                info!("    pos = {} instruction = {}", _pos, extract_instruction_tile_id);
                 */
            }
        }
    }

    //let ins = unique_hash_map.get_instructions_from_map(world_name, "1x1", "0");
    //warn!("{:?}", ins);
    // Print all entries in the UniqueHashMap
    //unique_hash_map.print_entries();

    // Write debug file
    //write_debug_file(world_name, &unique_hash_map);
}

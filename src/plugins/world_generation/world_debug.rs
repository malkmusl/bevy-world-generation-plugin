pub fn test_world(worlds: Res<Worlds>) {
    // Create some sample LevelTiles for each LevelLayer
    let level_tile_0_0 = LevelTile {
        x: 0,
        y: 0,
        instruction: "Move forward".to_string(),
        tileset_id: "grass_tileset".to_string(),
    };

    let level_tile_0_1 = LevelTile {
        x: 1,
        y: 0,
        instruction: "Turn left".to_string(),
        tileset_id: "grass_tileset".to_string(),
    };

    let level_tile_1_0 = LevelTile {
        x: 0,
        y: 1,
        instruction: "Jump".to_string(),
        tileset_id: "water_tileset".to_string(),
    };

    let level_tile_1_1 = LevelTile {
        x: 1,
        y: 1,
        instruction: "Attack".to_string(),
        tileset_id: "water_tileset".to_string(),
    };

    // Create LevelLayers containing the LevelTiles
    let layer_0 = LevelLayer {
        tiles: vec![level_tile_0_0, level_tile_0_1],
    };

    let layer_1 = LevelLayer {
        tiles: vec![level_tile_1_0, level_tile_1_1],
    };

    // Create a Level and insert it into the worlds hashmap
    let level1 = Level {
        name: "test_1".to_string(),
        layer_0,
        layer_1,
        layer_2: LevelLayer { tiles: vec![] }, // You can add more tiles here for the third layer if needed
    };

    // Create a second Level and insert it into the worlds hashmap
    let level_tile_2_0 = LevelTile {
        x: 0,
        y: 0,
        instruction: "Pick up item".to_string(),
        tileset_id: "dungeon_tileset".to_string(),
    };

    let level_tile_2_1 = LevelTile {
        x: 1,
        y: 0,
        instruction: "Open door".to_string(),
        tileset_id: "dungeon_tileset".to_string(),
    };

    let layer_2 = LevelLayer {
        tiles: vec![level_tile_2_0, level_tile_2_1],
    };

    let level2 = Level {
        name: "test_2".to_string(),
        layer_0: LevelLayer { tiles: vec![] }, // You can add more tiles here for the first layer if needed
        layer_1: LevelLayer { tiles: vec![] }, // You can add more tiles here for the second layer if needed
        layer_2,
    };

    // Insert the levels into the worlds hashmap
    worlds.insert_level(level1);
    worlds.insert_level(level2);

    // Accessing a specific level
    if let Some(level) = worlds.get_level("test_1") {
        // Accessing specific tiles in layer_0 of Level 1
        if let Some(tile) = worlds.get_tile_from_layer(&level.name, 0, 0 , 0) {
            println!(
                "Tile at position ({}, {}) - Instruction: {}, Tileset ID: {}",
                tile.x, tile.y, tile.instruction, tile.tileset_id
            );
        }
    }
}
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::RwLock;

use super::{
    level_plugin::{
        Level,
        LevelLayer, 
        LevelTile,
    }, 
    loader::{
        world_loader_plugin::WorldLoaderPlugin, 
        world_painter_plugin::WorldPainterPlugin,
    },};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        warn!("Initialize: WorldPlugin");
        app.add_plugins((
            WorldLoaderPlugin,
            WorldPainterPlugin,
        ));
        app.init_resource::<Worlds>()  // Add the Arc to Bevy's App resources
            .add_systems(PreStartup, test_world);
    }
}
#[derive(Default, Resource)]
pub struct Worlds {
    data: RwLock<HashMap<String, Level>>,
}
#[allow(unused_variables)]
#[allow(dead_code)]
impl Worlds {
    pub fn new() -> Self {
        Worlds {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert_level(&self, level: Level) {
        self.data.write().unwrap().insert(level.name.clone(), level);
    }

    pub fn insert_layer(&self, level_name: &str, layer_index: usize, layer: LevelLayer) {
        if let Some(mut level) = self.get_level_mut(level_name) {
            match layer_index {
                0 => level.layer_0 = layer,
                1 => level.layer_1 = layer,
                2 => level.layer_2 = layer,
                _ => (),
            }
        }
    }

    pub fn get_level(&self, level_name: &str) -> Option<Level> {
        let worlds = self.data.read().unwrap();
        worlds.get(level_name).cloned()
    }

    pub fn get_level_mut(&self, level_name: &str) -> Option<Level> {
        let mut worlds = self.data.write().unwrap();
        worlds.get_mut(level_name).cloned()
    }

    pub fn get_layer(&self, level_name: &str, layer_index: usize) -> Option<LevelLayer> {
        if let Some(level) = self.get_level(level_name) {
            match layer_index {
                0 => Some(level.layer_0.clone()),
                1 => Some(level.layer_1.clone()),
                2 => Some(level.layer_2.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_layer_mut(
        &self,
        level_name: &str,
        layer_index: usize,
    ) -> Option<LevelLayer> {
        if let Some(level) = self.get_level_mut(level_name) {
            match layer_index {
                0 => Some(level.layer_0.clone()),
                1 => Some(level.layer_1.clone()),
                2 => Some(level.layer_2.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_tile_from_layer(
        &self,
        level_name: &str,
        layer_index: usize,
        x: usize,
        y: usize,
    ) -> Option<LevelTile> {
        if let Some(layer) = self.get_layer(level_name, layer_index) {
            self.get_tile_at_layer(&layer, x, y)
        } else {
            None
        }
    }

    fn get_tile_at_layer(
        &self,
        layer: &LevelLayer,
        x: usize,
        y: usize,
    ) -> Option<LevelTile> {
        layer.tiles.iter().find(|tile| tile.x == x && tile.y == y).cloned()
    }
}


fn test_world() {
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
        name: "Level 1".to_string(),
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
        name: "Level 2".to_string(),
        layer_0: LevelLayer { tiles: vec![] }, // You can add more tiles here for the first layer if needed
        layer_1: LevelLayer { tiles: vec![] }, // You can add more tiles here for the second layer if needed
        layer_2,
    };

    // Insert the levels into the worlds hashmap
    let worlds = Worlds::new();
    worlds.insert_level(level1);
    worlds.insert_level(level2);

    // Accessing a specific level
    if let Some(level) = worlds.get_level("Level 1") {
        // Accessing specific tiles in layer_0 of Level 1
        if let Some(tile) = worlds.get_tile_from_layer(&level.name, 0, 0 , 0) {
            println!(
                "Tile at position ({}, {}) - Instruction: {}, Tileset ID: {}",
                tile.x, tile.y, tile.instruction, tile.tileset_id
            );
        }
    }
}

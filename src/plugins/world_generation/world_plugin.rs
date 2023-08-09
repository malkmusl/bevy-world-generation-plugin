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
        let _worlds = Worlds::new();
        app.add_plugins((
            WorldLoaderPlugin,
            WorldPainterPlugin,
        ));
        app.init_resource::<Worlds>().insert_resource(_worlds);  // Add the Arc to Bevy's App resources

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


// Define the structs
#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub layer_0: LevelLayer,
    pub layer_1: LevelLayer,
    pub layer_2: LevelLayer,
}

#[derive(Clone)]
pub struct LevelLayer {
    pub tiles: Vec<LevelTile>,
}

#[derive(Clone)]
pub struct LevelTile {
    pub x: usize,
    pub y: usize,
    pub instruction: String,
    pub tileset_id: String,
}
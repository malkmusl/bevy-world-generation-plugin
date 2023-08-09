// Define the structs
#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub layer_0: LevelLayer,
    pub layer_1: LevelLayer,
    pub layer_2: LevelLayer,
}
#[allow(dead_code)]
impl Level {
    pub fn new(name: String, layer_0: LevelLayer, layer_1: LevelLayer, layer_2: LevelLayer) -> Self {
        Level {
            name,
            layer_0,
            layer_1,
            layer_2,
        }
    }

    // Getters
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_layer_0(&self) -> &LevelLayer {
        &self.layer_0
    }

    pub fn get_layer_1(&self) -> &LevelLayer {
        &self.layer_1
    }

    pub fn get_layer_2(&self) -> &LevelLayer {
        &self.layer_2
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_layer_0(&mut self, layer: LevelLayer) {
        self.layer_0 = layer;
    }

    pub fn set_layer_1(&mut self, layer: LevelLayer) {
        self.layer_1 = layer;
    }

    pub fn set_layer_2(&mut self, layer: LevelLayer) {
        self.layer_2 = layer;
    }

    pub fn calculate_size(&self) -> usize {
        let mut max_x = 0;
        let mut max_y = 0;

        for tile in self.layer_0.tiles.iter() {
            max_x = max_x.max(tile.x);
            max_y = max_y.max(tile.y);
        }
        // Add 1 to account for 0-based indexing
        max_x + max_y + 1
    }
}


#[derive(Clone, Default)]
pub struct LevelLayer {
    pub tiles: Vec<LevelTile>,
}
#[allow(dead_code)]
impl LevelLayer {
    pub fn new(tiles: Vec<LevelTile>) -> Self {
        LevelLayer { tiles }
    }

    // Getters
    pub fn get_tiles(&self) -> &Vec<LevelTile> {
        &self.tiles
    }

    // Setters
    pub fn set_tiles(&mut self, tiles: Vec<LevelTile>) {
        self.tiles = tiles;
    }

    // Add a new tile
    pub fn add_tile(&mut self, tile: LevelTile) {
        self.tiles.push(tile);
    }

    pub fn find_tile_index(&self, x: usize, y: usize) -> Option<usize> {
        self.tiles.iter().position(|tile| tile.x == x && tile.y == y)
    }

    // Remove a tile at the given index
    pub fn remove_tile_at(&mut self, index: usize) -> Option<LevelTile> {
        if index < self.tiles.len() {
            Some(self.tiles.remove(index))
        } else {
            None
        }
    }

    // Update a tile at the given index
    pub fn update_tile_at(&mut self, index: usize, tile: LevelTile) -> Result<(), String> {
        if index < self.tiles.len() {
            self.tiles[index] = tile;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }
}


#[derive(Clone)]
pub struct LevelTile {
    pub x: usize,
    pub y: usize,
    pub instruction: String,
    pub tileset_id: String,
}

#[allow(dead_code)]
impl LevelTile {
    pub fn new(x: usize, y: usize, instruction: String, tileset_id: String) -> Self {
        LevelTile {
            x,
            y,
            instruction,
            tileset_id,
        }
    }

    // Getters
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_instruction(&self) -> &String {
        &self.instruction
    }

    pub fn get_tileset_id(&self) -> &String {
        &self.tileset_id
    }

    // Setters
    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn set_instruction(&mut self, instruction: String) {
        self.instruction = instruction;
    }

    pub fn set_tileset_id(&mut self, tileset_id: String) {
        self.tileset_id = tileset_id;
    }
}

use bevy::prelude::*;
use bevy::prelude::Commands;
use rand::Rng;

const GRID_SIZE: usize = 10; // Adjust this to change the grid size

#[derive(Component)]
pub struct Tile;

pub struct WorldPainterPlugin;

impl Plugin for WorldPainterPlugin {
    fn build(&self, app: &mut App) {
        warn!("Initialize: WorldPainterPlugin");
        app.add_systems(PreStartup,(
            init_tile_map,
            ));
        app.add_systems(Startup, (
            spawn_tile.after(init_tile_map),
        ));
    }
}

fn init_tile_map(mut commands: Commands) {
    commands.insert_resource(TileMap::new());
}

pub fn spawn_tile(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut tile_map: ResMut<TileMap>,
) {
    let texture_handle = asset_server.load("tileset/Outside.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 502, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Number of tiles to spawn       
    const NUM_TILES_TO_SPAWN: usize = 65536;

    // Scale factor for the grid
    const GRID_SCALE: f64 = 1.6; // Adjust this value to control the overall size of the grid

    // Calculate the grid size based on the number of tiles to spawn
    let mut grid_size = (NUM_TILES_TO_SPAWN as f32).sqrt() as usize;

    // Make sure the grid size is even to guarantee the center is in the center of a cell
    if grid_size % 2 != 0 {
        grid_size += 1;
    }

    // Calculate the tile size based on the grid scale
    let tile_size = 32.0 * GRID_SCALE; // Adjust this value to control the overall size of the grid

    // Calculate the starting positions for spawning tiles
    let start_row = grid_size / 2;
    let start_col = grid_size / 2;
    let mut row = start_row;
    let mut col = start_col;

    for _ in 0..NUM_TILES_TO_SPAWN {
        let tile_number = rand::thread_rng().gen_range(2..6);

        // Check if the position is already occupied
        while tile_map.is_occupied(row, col) {
            col += 1;
            if col >= grid_size {
                col = 0;
                row += 1;
            }
            if row >= grid_size {
                row = 0;
            }
        }

        // Mark the position as occupied and spawn the tile
        tile_map.mark_occupied(row, col);

        // Calculate the original position with scaling
        let original_position = calculate_position(row, col, grid_size, tile_size);
        let original_position_f32: Vec3 = original_position.into();
        let _position = original_position_f32 * GRID_SCALE as f32;

        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(tile_number),
            transform: Transform::from_translation(original_position_f32)
                .mul_transform(Transform::from_scale(Vec3::splat(GRID_SCALE as f32))),// Set the sprite size based on the tile size
            ..Default::default()
        });

        col += 1;
        if col >= grid_size {
            col = 0;
            row += 1;
        }
        if row >= grid_size {
            row = 0;
        }
    }
}

pub fn calculate_position(row: usize, col: usize, grid_size: usize, tile_size: f64) -> Vec3 {
    let half_grid_size = grid_size as f64 / 2.0;
    let x = (col as i32 - half_grid_size as i32) as i32;
    let y = (row as i32 - half_grid_size as i32) as i32;
    Vec3::new(x as f32, y as f32, 0.0) * tile_size as f32
}


#[derive(Resource)]
pub struct TileMap {
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl TileMap {
    fn new() -> Self {
        Self {
            grid: [[false; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        if let Some(row_cells) = self.grid.get(row) {
            if let Some(&occupied) = row_cells.get(col) {
                return occupied;
            }
        }
        false
    }

    fn mark_occupied(&mut self, row: usize, col: usize) {
        if let Some(row_cells) = self.grid.get_mut(row) {
            if let Some(occupied) = row_cells.get_mut(col) {
                *occupied = true;
            }
        }
    }
}

impl Default for TileMap {
    fn default() -> Self {
        Self::new()
    }
}

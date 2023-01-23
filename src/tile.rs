
use bevy::prelude::*;
use bevy::math::*;
use bevy_mod_picking::*;

use crate::grid::Grid;

#[derive(Clone, Copy, Default, Debug)]
pub enum TileType {
    #[default]
    None, Start, End, Wall
}
impl TileType {
    pub fn color(&self) -> Color {
        match *self {
            TileType::None => BG_COLOR,
            TileType::Wall => WALL_COLOR,
            TileType::Start => START_COLOR,
            TileType::End => END_COLOR,
        }
    }
}
const BG_COLOR: Color = Color::rgb(224./255., 251./255., 252./255.);
const WALL_COLOR: Color = Color::rgb(41./255., 50./255., 65./255.);
const START_COLOR: Color = Color::SEA_GREEN;
const END_COLOR: Color = Color::RED;
const CLOSED_COLOR: Color = Color::rgb(152./255., 193./255., 217./255.);
const OPEN_COLOR: Color = Color::rgb(61./255., 90./255., 128./255.);


#[derive(Clone, Copy)]
pub struct Tile {
    pub entity: Entity, //Entity containing the sprite to be rendered
    pub position: (usize, usize), //(x, y) position of the tile in the grid
    pub parent: Option<(usize, usize)>, //Optional (x, y) position of the tile's parent
    pub tile_type: TileType,
}
impl Tile {
    pub fn new(entity: Entity, position: (usize, usize)) -> Self {
        Self {entity, position, parent: None, tile_type: TileType::default()}
    }
    pub fn from(entity: Entity, position: (usize, usize), parent: Option<(usize, usize)>, tile_type: TileType) -> Self {
        Self {entity, position, parent, tile_type}
    }
    pub fn default() -> Self {
        Self {
            entity: Entity::from_bits(0),
            position: (0, 0),
            parent: None,
            tile_type: TileType::default()
        }
    }

    //Sets the Tile's type and sets the TisualTile's color
    pub fn set_type_smart(
        &mut self,
        new_type: Option<TileType>,
        mut sprite: &mut Sprite
    ) {
        match new_type {
            Some(t) => {
                self.tile_type = t;
                sprite.color = t.color();
            },
            None => {
                self.tile_type = match self.tile_type {
                    TileType::None => TileType::Wall,
                    TileType::Wall => TileType::None,
                    _ => self.tile_type
                };
                sprite.color = self.tile_type.color();
            },
        }
    }
}


#[derive(Component)]
pub struct VisualTile {
    pub x: usize, pub y: usize
}
impl VisualTile {
    pub fn new(position: (usize, usize)) -> Self {
        Self {x: position.0, y: position.1}
    }

    pub fn set_color_smart(sprite: &mut Sprite, color: Option<Color>) {
        sprite.color = match color {
            Some(c) => c,
            None => if sprite.color==WALL_COLOR {BG_COLOR} else {WALL_COLOR},
        }
    }
}

#[derive(Bundle)]
pub struct VisualTileBundle {
    visual_tile: VisualTile,
    sprite_bundle: SpriteBundle,
    mesh: Handle<Mesh>,
    pickable_bundle: PickableBundle,
}
impl VisualTileBundle {
    pub fn new(position: (usize, usize), translation: Vec3, size: f32, mesh_assets: &mut ResMut<Assets<Mesh>>, tile_type: TileType) -> Self {
        let visual_tile = VisualTile::new(position);
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {color: tile_type.color(), ..default()},
            transform: Transform { translation, scale: vec3(size, size, 1.), ..default() },
            ..default()
        };
        let mesh = mesh_assets.add(Mesh::from(shape::Quad::default()));
        let pickable_bundle = PickableBundle::default();
        Self { visual_tile, sprite_bundle, mesh, pickable_bundle }
    }
}

pub fn process_tile_click_events(
    mut event_reader: EventReader<PickingEvent>,
    mut sprite_query: Query<(&mut Sprite, &mut VisualTile), With<VisualTile>>,
    mut grid_query: Query<&mut Grid>
) {
    let mut grid = grid_query.get_single_mut().unwrap();
    for event in event_reader.iter() {
        match *event {
            PickingEvent::Clicked(e) => {
                let (mut sprite, visual_tile) = sprite_query.get_mut(e).unwrap();
                grid.grid[visual_tile.y][visual_tile.x].set_type_smart(None, sprite.as_mut());
                
            },
            _ => ()
        }
    }
}
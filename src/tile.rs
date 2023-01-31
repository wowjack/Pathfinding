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
pub const BG_COLOR: Color = Color::rgb(224./255., 251./255., 252./255.);
pub const WALL_COLOR: Color = Color::rgb(41./255., 50./255., 65./255.);
pub const START_COLOR: Color = Color::SEA_GREEN;
pub const END_COLOR: Color = Color::RED;
pub const CLOSED_COLOR: Color = Color::rgb(152./255., 193./255., 217./255.);
pub const OPEN_COLOR: Color = Color::rgb(61./255., 90./255., 128./255.);
pub const PATH_COLOR: Color = Color::rgb(238./255., 108./255., 77./255.);


#[derive(Clone, Copy)]
pub struct Tile {
    pub entity: Entity, //Entity containing the sprite to be rendered
    pub position: (usize, usize), //(x, y) position of the tile in the grid
    pub parent: Option<(usize, usize)>, //Optional (x, y) position of the tile's parent
    pub tile_type: TileType,
}
impl Tile {
    pub fn new(entity: Entity, position: (usize, usize), parent: Option<(usize, usize)>, tile_type: TileType) -> Self {
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

    //Sets the Tile's type and sets the VisualTile's color
    pub fn set_type(&mut self, new_type: TileType, mut sprite: &mut Sprite) {
        self.tile_type = new_type;
        sprite.color = new_type.color();
    }

    //Swap tile between wall and none if new_type is None, do not overwrite start or end
    pub fn click(&mut self, mut sprite: &mut Sprite, new_type: Option<TileType>) -> TileType {
        match new_type {
            None => {
                self.tile_type = match self.tile_type {
                    TileType::None => TileType::Wall,
                    TileType::Wall => TileType::None,
                    _ => self.tile_type 
                };
            },
            Some(new_type) => {
                self.tile_type = match self.tile_type {
                    TileType::Wall | TileType::None => new_type,
                    _ => self.tile_type,
                }
            }
        }
        sprite.color = self.tile_type.color();
        return self.tile_type;
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
    mut sprite_query: Query<(&mut Sprite, &mut VisualTile)>,
    mut grid_query: Query<&mut Grid>,
    mut hover_tile_type: Local<TileType>,
    mouse_state: Res<Input<MouseButton>>
) {

    if !mouse_state.pressed(MouseButton::Left) {*hover_tile_type = TileType::default()}
    let mut grid = grid_query.get_single_mut().unwrap();
    for event in event_reader.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                let (mut sprite, visual_tile) = sprite_query.get_mut(*e).unwrap();
                *hover_tile_type = grid.grid[visual_tile.y][visual_tile.x].click(sprite.as_mut(), None);
            },
            PickingEvent::Hover(hover_event) => {
                match hover_event {
                    HoverEvent::JustEntered(e) => {
                        if !mouse_state.pressed(MouseButton::Left) {continue}
                        let (mut sprite, visual_tile) = sprite_query.get_mut(*e).unwrap();
                        if (visual_tile.x, visual_tile.y) == grid.start || (visual_tile.x, visual_tile.y) == grid.end {continue}
                        match *hover_tile_type {
                            TileType::End => grid.set_end((visual_tile.x, visual_tile.y), &mut sprite_query),
                            TileType::Start => grid.set_start((visual_tile.x, visual_tile.y), &mut sprite_query),
                            TileType::None | TileType::Wall => {
                                grid.grid[visual_tile.y][visual_tile.x].click(sprite.as_mut(), Some(*hover_tile_type));
                            }
                        }
                    },
                    _ => ()
                }
            },
            _ => (),
        }
    }
}


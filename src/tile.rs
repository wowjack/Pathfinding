
use bevy::prelude::*;
use bevy::math::*;
use bevy::sprite;
use bevy_mod_picking::*;

#[derive(Clone, Copy, Default)]
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

    pub fn set_color(entity: Entity, mut sprite_query: Query<&mut Sprite, With<VisualTile>>, color: Option<Color>) {
        let sprite = sprite_query.get_mut(entity);
        match (sprite, color) {
            (Ok(mut sprite), Some(color)) => sprite.color = color,
            (Ok(mut sprite), None) => {
                todo!("Implement smart color swapping");
            },
            _ => return,
        }
    }
}


#[derive(Component)]
pub struct VisualTile {
    x: usize, y: usize
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
    pub fn new(position: (usize, usize), translation: Vec3, size: f32, mut mesh_assets: &mut ResMut<Assets<Mesh>>, tile_type: TileType) -> Self {
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
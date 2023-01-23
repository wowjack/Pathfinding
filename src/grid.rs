use crate::tile::*;
use bevy::prelude::*;
use bevy::math::*;

pub enum GridEvent {
    Resize(usize)
}


#[derive(Default, Component)]
pub struct Grid {
    pub grid: Vec<Vec<Tile>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
    visual_size: f32,
    grid_size: usize
}
impl Grid {
    pub fn spawn_grid(commands: &mut Commands, mesh_assets: &mut ResMut<Assets<Mesh>>, grid_size: usize, visual_size: f32, translation: Vec3) {
        if grid_size==0 {return}

        let mut grid: Vec<Vec<Tile>> = vec![];
        let sprite_size = Grid::sprite_size(visual_size, grid_size);

        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(translation),
            ..default()
        })
        .with_children(|builder| {
            for y in 0..grid_size {
                let mut row: Vec<Tile> = vec![];
                for x in 0..grid_size {
                    let translation: Vec3 = vec3(x as f32 * (sprite_size+1.), y as f32 * (sprite_size+1.), 0.) + sprite_size/2.;
                    let tile_type = if x==1 && y==1 {TileType::Start} else if x==grid_size-2 && x==y {TileType::End} else {TileType::None};
                    let entity = builder.spawn(VisualTileBundle::new((x, y), translation, sprite_size, mesh_assets, tile_type)).id();
                    //println!("tile: {:?}, color: {:?}, type: {:?}", (x, y), tile_type.color(), tile_type);
                    row.push(Tile::from(entity, (x, y), None, tile_type));
                }
                grid.push(row);
            }
        }).insert(Grid {
            grid, start: (1, 1), end: (grid_size-2, grid_size-2), visual_size, grid_size
        });
    }

    pub fn resize(entity: Entity, commands: &mut Commands, mesh_assets: &mut ResMut<Assets<Mesh>>, grid_query: &mut Query<&mut Grid>, new_size: usize) {
        if new_size < 5 {return}
        commands.entity(entity).despawn_descendants();

        let mut grid = grid_query.get_mut(entity).unwrap();
        let old_size = grid.grid_size;
        grid.grid[old_size-2][old_size-2].tile_type = TileType::None;
        grid.grid.resize(new_size, vec![Tile::default(); new_size]);
        grid.grid.iter_mut().for_each(|row| {
            row.resize(new_size, Tile::default());
        });
        grid.grid_size = new_size;
        grid.grid[new_size-2][new_size-2].tile_type = TileType::End;


        let sprite_size = Grid::sprite_size(grid.visual_size, grid.grid_size);

        commands.entity(entity).add_children(|builder| {
            for y in 0..new_size {
                for x in 0..new_size {
                    let translation =  vec3(x as f32 * (sprite_size+1.), y as f32 * (sprite_size+1.), 0.) + sprite_size/2.;
                    let e = builder.spawn(VisualTileBundle::new((x, y), translation, sprite_size, mesh_assets, grid.grid[y][x].tile_type)).id();
                    grid.grid[y][x].entity = e;
                    grid.grid[y][x].position = (x, y);
                }
            }
        });
    }

    //getters
    pub fn grid_size(&self) -> usize {self.grid_size}
    pub fn visual_size(&self) -> f32 {self.visual_size}
    pub fn sprite_size(visual_size: f32, grid_size: usize) -> f32 {(visual_size - grid_size as f32) / (grid_size as f32)}
}


pub fn process_grid_events(mut commands: Commands, mut event_reader: EventReader<GridEvent>, grid_entity_query: Query<Entity, With<Grid>>, mut grid_query: Query<&mut Grid>, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let grid_entity = grid_entity_query.get_single().unwrap();
    for event in event_reader.iter() {
        match *event {
            GridEvent::Resize(size) => Grid::resize(grid_entity, &mut commands, &mut mesh_assets, &mut grid_query, size),
        }
    }
}
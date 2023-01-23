use crate::tile::*;
use bevy::prelude::*;
use bevy::math::*;

pub enum GridEvent {
    Resize(usize),
    Clear,
    Reset,
    Solve,
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
                    row.push(Tile::new(entity, (x, y), None, tile_type));
                }
                grid.push(row);
            }
        }).insert(Grid {
            grid, start: (1, 1), end: (grid_size-2, grid_size-2), visual_size, grid_size
        });
    }

    pub fn resize(
        entity: Entity,
        commands: &mut Commands,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        grid_query: &mut Query<&mut Grid>,
        new_size: usize,
        sprite_query: &mut Query<(&mut Sprite, &mut VisualTile)>
    ) {
        if new_size < 5 {return}
        commands.entity(entity).despawn_descendants();

        let mut grid = grid_query.get_mut(entity).unwrap();
        grid.grid.resize(new_size, vec![Tile::default(); new_size]);
        grid.grid.iter_mut().for_each(|row| {
            row.resize(new_size, Tile::default());
        });
        grid.grid_size = new_size;

        //reset start and end tiles if they were deleted
        if grid.start.0>=grid.grid_size || grid.start.1>=grid.grid_size {
            grid.set_start((1, 1), sprite_query);
        } else if grid.end.0>=grid.grid_size || grid.end.1>=grid.grid_size {
            grid.set_end((new_size-2, new_size-2), sprite_query);
        }


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

    //reset the color of every tile except for Wall, Start, and End
    pub fn clear() {
        todo!()
    }

    //reset the entire grid back to its original state
    pub fn reset() {
        todo!()
    }

    //resets type and color of previous start and sets new start
    pub fn set_start(&mut self, new: (usize, usize), sprite_query: &mut Query<(&mut Sprite, &mut VisualTile)>) {
        if new.0>=self.grid_size || new.1>=self.grid_size {return;}
        if self.start.0<self.grid_size && self.start.1<self.grid_size {
            let (mut sprite, _visual_tile) = sprite_query.get_mut(self.grid[self.start.1][self.start.0].entity).unwrap();
            self.grid[self.start.1][self.start.0].set_type(TileType::None, sprite.as_mut());
        }
        self.start = new;
        let (mut sprite, _visual_tile) = sprite_query.get_mut(self.grid[self.start.1][self.start.0].entity).unwrap();
        self.grid[self.start.1][self.start.0].set_type(TileType::Start, sprite.as_mut());
    }

    //resets type and color of previous end and sets new end
    pub fn set_end(&mut self, new: (usize, usize), sprite_query: &mut Query<(&mut Sprite, &mut VisualTile)>) {
        if new.0>self.grid_size || new.1>self.grid_size {return;}
        if self.end.0<self.grid_size && self.end.1<self.grid_size {
            let (mut sprite, _visual_tile) = sprite_query.get_mut(self.grid[self.end.1][self.end.0].entity).unwrap();
            self.grid[self.end.1][self.end.0].set_type(TileType::None, sprite.as_mut());
        }
        self.end = new;
        let (mut sprite, _visual_tile) = sprite_query.get_mut(self.grid[self.end.1][self.end.0].entity).unwrap();
        self.grid[self.end.1][self.end.0].set_type(TileType::End, sprite.as_mut());
    }

    //calculate the size of tile sprites
    pub fn sprite_size(visual_size: f32, grid_size: usize) -> f32 {(visual_size - grid_size as f32) / (grid_size as f32)}
}


pub fn process_grid_events(
    mut commands: Commands,
    mut event_reader: EventReader<GridEvent>,
    grid_entity_query: Query<Entity, With<Grid>>,
    mut grid_query: Query<&mut Grid>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut sprite_query: Query<(&mut Sprite, &mut VisualTile)>
) {
    let grid_entity = grid_entity_query.get_single().unwrap();
    for event in event_reader.iter() {
        match *event {
            GridEvent::Resize(size) => {
                Grid::resize(grid_entity, &mut commands, &mut mesh_assets, &mut grid_query, size, &mut sprite_query);
            },
            GridEvent::Clear => {
                Grid::clear();
            },
            GridEvent::Reset => {
                Grid::reset();
            },
            GridEvent::Solve => {
                todo!()
            }
        }
    }
}
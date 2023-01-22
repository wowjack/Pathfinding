#![allow(non_snake_case)]


use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle};
use grid_update::*;
use gui::*;
use solver::*;

mod gui;
mod grid_update;
mod solver;

const TILE_SIZE: f32 = 23.4;
const GRID_SIZE: usize = 32;

fn main() {
    App::new()
        .add_event::<FastTileEvent>()
        .add_event::<SlowTileEvent>()
        .add_event::<GridEvent>()
        .init_resource::<SlowTileUpdateBuffer>()
        .init_resource::<UpdateTimer>()
        .init_resource::<Algorithm>()
        .init_resource::<ColorPalette>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pathfinding".to_string(),
                width: 1200.,
                height: 790.,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_system(process_click_events)
        .add_system(gui)
        .add_system(save_tile_color_events)
        .add_system(process_fast_tile_events)
        .add_system(process_slow_tile_events)
        .add_system(process_grid_events)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Res<Windows>,
    mut event_writer: EventWriter<FastTileEvent>,
    colors: Res<ColorPalette>
) {
    let window = windows.get_primary().expect("Failed to find primary window");
    commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default()));

    let mut entity_grid: Vec<Vec<TileRef>> = vec![];
    //create the game state and all color tiles as children
    use bevy::math::*;
    commands.spawn(SpriteBundle {
            transform: Transform::from_translation(vec3(-1.*window.width()/2. + TILE_SIZE, -1.*window.height()/2. + TILE_SIZE, 0.)),
            ..default()
        })
        .with_children(|builder| {
            for i in 0..GRID_SIZE {
                entity_grid.push(Vec::new());
                for j in 0..GRID_SIZE {
                    let e = builder.spawn(SpriteBundle {
                        transform: Transform {
                            translation: vec3(i as f32*(TILE_SIZE+0.5), j as f32*(TILE_SIZE+0.5), 0.),
                            scale: vec3(TILE_SIZE, TILE_SIZE, 1.),
                            ..default()
                        },
                        sprite: Sprite {
                            color: colors.bg.clone(),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(meshes.add(Mesh::from(shape::Quad::default())))
                    .insert(PickableBundle::default())
                    .insert(GridTile(i, j)).id();
                    entity_grid[i].push(TileRef {
                        entity: e,
                        position: (i, j),
                        tile_type: TileType::None,
                        parent: None
                    });
                }
            }
            entity_grid[1][1].tile_type = TileType::Start;
            event_writer.send(FastTileEvent(entity_grid[1][1].entity, Some(colors.start)));
            entity_grid[GRID_SIZE-2][GRID_SIZE-2].tile_type = TileType::End;
            event_writer.send(FastTileEvent(entity_grid[GRID_SIZE-2][GRID_SIZE-2].entity, Some(colors.end)));

        }).insert(GridState {
            grid: entity_grid,
            start: (1, 1),
            end: (GRID_SIZE-2, GRID_SIZE-2)
        });
}


#[derive(Component)]
pub struct GridState{
    grid: Vec<Vec<TileRef>>,
    start: (usize, usize),
    end: (usize, usize)
}

#[derive(Clone, Copy)]
pub struct TileRef {
    entity: Entity,
    position: (usize, usize),
    tile_type: TileType,
    parent: Option<(usize, usize)>,
}

#[derive(Clone, Copy)]
pub enum TileType {
    Start, End, Wall, None
}
#[derive(Component)]
pub struct GridTile(usize, usize);

#[derive(Resource, Clone, Copy)]
pub struct ColorPalette {
    pub bg: Color,
    pub wall: Color,
    pub open: Color,
    pub closed: Color,
    pub path: Color,
    pub start: Color,
    pub end: Color
}
impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            bg: Color::hex("e0fbfc").unwrap(),
            wall: Color::hex("293241").unwrap(),
            open: Color::hex("3d5a80").unwrap(),
            closed: Color::hex("98c1d9").unwrap(),
            path: Color::hex("ee6c4d").unwrap(),
            start: Color::SEA_GREEN,
            end: Color::RED,
        }
    }
}
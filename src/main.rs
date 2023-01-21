#![allow(non_snake_case)]


use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle, PickingEvent, HoverEvent};
use grid_update::*;
use gui::*;

mod gui;
mod grid_update;

const TILE_SIZE: f32 = 22.;
const GRID_SIZE: usize = 32;

fn main() {
    App::new()
        .add_event::<FastTileEvent>()
        .add_event::<SlowTileEvent>()
        .init_resource::<SlowTileUpdateBuffer>()
        .init_resource::<UpdateTimer>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pathfinding".to_string(),
                width: 1200.,
                height: 790.,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_system(allow_clicking)
        .add_system(gui)
        .add_system(save_tile_color_events)
        .add_system(process_fast_tile_events)
        .add_system(process_slow_tile_events)
        .run();
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, windows: Res<Windows>) {
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
                            translation: vec3(i as f32*(TILE_SIZE+2.), j as f32*(TILE_SIZE+2.), 0.),
                            scale: vec3(TILE_SIZE, TILE_SIZE, 1.),
                            ..default()
                        },
                        sprite: Sprite {
                            color: Color::WHITE,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(meshes.add(Mesh::from(shape::Quad::default())))
                    .insert(PickableBundle::default())
                    .insert(GridTile).id();
                    entity_grid[i].push(TileRef {
                        entity: e,
                        position: (i, j),
                        tile_type: TileType::None
                    });
                }
            }
        }).insert(GameState {
            grid: entity_grid
        });
}


#[derive(Component)]
pub struct GameState{
    grid: Vec<Vec<TileRef>>
}

pub struct TileRef {
    entity: Entity,
    position: (usize, usize),
    tile_type: TileType
}
pub enum TileType {
    Start, End, Wall, None
}

#[derive(Component)]
pub struct GridTile;

fn allow_clicking(
    mut events: EventReader<PickingEvent>,
    mouse: Res<Input<MouseButton>>,
    mut click_event_writer: EventWriter<FastTileEvent>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            click_event_writer.send(FastTileEvent(*e, None));
        }
        if let PickingEvent::Hover(HoverEvent::JustEntered(e)) = event {
            if mouse.pressed(MouseButton::Left) {
                click_event_writer.send(FastTileEvent(*e, Some(Color::BLACK)));
            }
        }
        
    }
}
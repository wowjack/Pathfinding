#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle, DebugEventsPickingPlugin, PickingEvent};
use gui::*;

mod gui;

const TILE_SIZE: f32 = 22.;

fn main() {
    App::new()
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
        .add_system(events)
        .add_system(gui)
        .run();
}

fn init(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, windows: Res<Windows>) {
    let window = windows.get_primary().expect("Failed to find primary window");
    commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default()));

    //create the game state and all color tiles as children
    use bevy::math::*;
    commands.spawn(GameState{})
        .insert(SpriteBundle {
            transform: Transform::from_translation(vec3(-1.*window.width()/2. + TILE_SIZE, -1.*window.height()/2. + TILE_SIZE, 0.)),
            ..default()
        })
        .with_children(|builder| {
            for i in 0..32 {
                for j in 0..32 {
                    builder.spawn(SpriteBundle {
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
                    .insert(PickableBundle::default());
                }
            }
        });
}


#[derive(Component)]
struct GameState {

}

fn events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            println!("{:?}", e);
        }
    }
}

/*
Visual tiles will be child components of the gamestate

*/
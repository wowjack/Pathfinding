#![allow(non_snake_case)]


use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle};
use gui::*;
use grid::*;
use tile::*;

mod gui;
mod grid;
mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pathfinding".to_string(),
                width: 1309.,
                height: 900.,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_event::<GridEvent>()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_system(gui)
        .add_system(process_grid_events)
        .add_system(process_tile_click_events)
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().expect("Failed to find primary window");
    commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default()));

    //create the grid state and visual tiles
    let bottom_left = bevy::math::vec3(-1.*window.width()/2., -1.*window.height()/2., 0.);
    Grid::spawn_grid(&mut commands, &mut meshes, 10, window.height(), bottom_left);
    
}

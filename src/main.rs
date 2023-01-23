#![allow(non_snake_case)]


use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};
use gui::*;
use grid::*;
use solver::SolverState;
use tile::*;

mod gui;
mod grid;
mod tile;
mod solver;

fn main() {

    let mut default_grid_size: usize = 20;

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
        .init_resource::<SolverState>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_system(move |ctx: ResMut<EguiContext>, grid_event_writer: EventWriter<GridEvent>| {
            gui(ctx, grid_event_writer, &mut default_grid_size);
        })
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
    Grid::spawn_grid(&mut commands, &mut meshes, 20, window.height(), bottom_left);
    
}

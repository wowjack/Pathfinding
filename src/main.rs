#![allow(non_snake_case)]

//Dragging start/end over end/start deletes one

use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle};
use gui::*;
use grid::*;
use solve_buffer::{SolveBuffer, process_update_buffer_system, UpdateTimer};
use solver::SolverState;
use tile::*;

mod gui;
mod grid;
mod tile;
mod solver;
mod solve_buffer;

fn main() {

    let mut default_grid_size: usize = 20;
    let mut solve_speed_divisor: f32 = 1.;

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
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .init_resource::<SolverState>()
        .init_resource::<SolveBuffer>()
        .init_resource::<UpdateTimer>()
        .add_event::<GridEvent>()
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(bevy_egui:: EguiPlugin)
        .add_startup_system(init)
        .add_system(move |ctx: ResMut<EguiContext>, grid_event_writer: EventWriter<GridEvent>, mut update_timer: ResMut<UpdateTimer>| {
            gui(ctx, grid_event_writer, &mut default_grid_size, &mut solve_speed_divisor, update_timer);
        })
        .add_system(process_grid_events)
        .add_system(process_tile_click_events)
        .add_system(process_update_buffer_system)
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

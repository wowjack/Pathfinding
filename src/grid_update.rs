use std::{collections::{VecDeque}};

use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};

use crate::{GridTile, GridState, TileType, solver::{Algorithm, start_solve}, GRID_SIZE, ColorPalette};

/*
Functions to read events from the solver system and write the changes to the tiles

How fast the solve goes depends on how quickly this system writes updates to the tiles.
This allows the speed of the solve to be changed.
This also allows the actual solve to happen much faster than its displayed.
*/

#[derive(Resource, Default)]
pub struct SlowTileUpdateBuffer(pub VecDeque<Vec<SlowTileEvent>>);

#[derive(Clone)]
pub struct SlowTileEvent(pub Entity, pub Color);

#[derive(Clone)]
pub struct FastTileEvent(pub Entity, pub Option<Color>);

#[derive(Clone)]
pub enum GridEvent {
    Reset, Clear, StartSolve
}

//Reads all the color change events for a frame and consolidates them into an event vec
pub fn save_tile_color_events(mut tile_update_list: ResMut<SlowTileUpdateBuffer>, mut event_reader: EventReader<SlowTileEvent>) {
    let mut event_vec: Vec<SlowTileEvent> = vec![];
    //populate map with events
    for e in event_reader.iter() {
        event_vec.push(e.clone());
    }
    if !event_vec.is_empty() {
        tile_update_list.0.push_back(event_vec);
    }
}

#[derive(Resource)]
pub struct UpdateTimer(pub Timer);
impl Default for UpdateTimer {
    fn default() -> Self {
        Self(Timer::new(std::time::Duration::from_millis(1), TimerMode::Repeating))
    }
}
pub fn process_slow_tile_events(mut tile_query: Query<&mut Sprite, With<GridTile>>, mut tile_update_list: ResMut<SlowTileUpdateBuffer>, time: Res<Time>, mut update_timer: ResMut<UpdateTimer>) {
    //periodically change the specified tile's color
    update_timer.0.tick(time.delta());

    if update_timer.0.finished() {
        if let Some(events) = tile_update_list.0.pop_front() {
            for event in events.iter() {
                tile_query.get_mut(event.0).unwrap().color = event.1;
            }
        }
    }
}

pub fn process_fast_tile_events(
    mut tile_query: Query<(&mut Sprite, &GridTile)>,
    mut event_reader: EventReader<FastTileEvent>,
    mut grid_query: Query<&mut GridState>,
    colors: Res<ColorPalette>
) {
    for event in event_reader.iter() {
        let mut t = tile_query.get_mut(event.0).unwrap();
        let c = t.0.color.clone();
        t.0.color = match event.1 {
            Some(c) => c,
            None => if c == colors.wall {colors.bg} else {colors.wall}
        };
        //Set tile type in game state TileRef grid
        let mut game = grid_query.get_single_mut().unwrap();
        if t.0.color == colors.wall {
            game.grid[t.1.0][t.1.1].tile_type = TileType::Wall; 
        } else if t.0.color == colors.bg {
            game.grid[t.1.0][t.1.1].tile_type = TileType::None; 
        }
    }
}

pub fn process_grid_events(
    mut event_reader: EventReader<GridEvent>,
    alg: Res<Algorithm>,
    mut tile_sprite_query: Query<&mut Sprite, With<GridTile>>,
    mut game_query: Query<&mut GridState>,
    mut buffer: ResMut<SlowTileUpdateBuffer>,
    colors: Res<ColorPalette>
) {
    let mut game = game_query.get_single_mut().unwrap();
    let colors = colors.as_ref();   
    for event in event_reader.iter() {
        match event {
            GridEvent::Clear => {
                game.grid.iter_mut().enumerate().for_each(|(i, row)| {
                    row.iter_mut().enumerate().for_each(|(j, tile_ref)| {
                        let mut sprite = tile_sprite_query.get_mut(tile_ref.entity).unwrap();
                        match tile_ref.tile_type {
                            TileType::None => sprite.color = colors.bg,
                            _ => (),
                        }
                    });
                });
            },
            GridEvent::Reset => {
                game.end = (GRID_SIZE-2, GRID_SIZE-2); game.start = (1, 1);
                game.grid.iter_mut().enumerate().for_each(|(i, row)| {
                    row.iter_mut().enumerate().for_each(|(j, tile_ref)| {
                        let mut sprite = tile_sprite_query.get_mut(tile_ref.entity).unwrap();
                        if i==j && i==1 {sprite.color = colors.start; tile_ref.tile_type = TileType::Start}
                        else if i==j && i==GRID_SIZE-2 {sprite.color = colors.end; tile_ref.tile_type = TileType::End}
                        else {sprite.color = colors.bg; tile_ref.tile_type = TileType::None}
                    })
                });
            },
            GridEvent::StartSolve => {
                start_solve(&alg, buffer.as_mut(), game.as_mut(), colors);
            }
        }
    }
}

pub fn process_click_events(
    mut events: EventReader<PickingEvent>,
    mouse: Res<Input<MouseButton>>,
    mut click_event_writer: EventWriter<FastTileEvent>,
    tile_query: Query<(&Sprite, &GridTile)>,
    mut hover_color: Local<Color>,
    colors: Res<ColorPalette>
) {
    for event in events.iter() {
        match event {
            PickingEvent::Clicked(e) => {
                *hover_color = if tile_query.get(*e).unwrap().0.color==colors.wall {colors.bg} else {colors.wall};
                click_event_writer.send(FastTileEvent(*e, Some(*hover_color)));
            },
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => {
                if mouse.pressed(MouseButton::Left) {
                    click_event_writer.send(FastTileEvent(*e, Some(*hover_color)));
                }
            },
            _ => (),
        }
    }
}
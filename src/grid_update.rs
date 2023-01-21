use std::{collections::{VecDeque}};

use bevy::prelude::*;

use crate::{GridTile, GameState, TileType};


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

pub fn process_fast_tile_events(mut tile_query: Query<(&mut Sprite, &GridTile)>, mut event_reader: EventReader<FastTileEvent>, mut game_query: Query<&mut GameState>) {
    for event in event_reader.iter() {
        let mut t = tile_query.get_mut(event.0).unwrap();
        let c = t.0.color.clone();
        t.0.color = match event.1 {
            Some(c) => c,
            None => if c == Color::BLACK {Color::WHITE} else {Color::BLACK}
        };
        //Set tile type in game state TileRef grid
        let mut game = game_query.get_single_mut().unwrap();
        if t.0.color == Color::BLACK {
            game.grid[t.1.0][t.1.1].tile_type = TileType::Wall; 
        } else if t.0.color == Color::WHITE {
            game.grid[t.1.0][t.1.1].tile_type = TileType::None; 
        }
    }
}


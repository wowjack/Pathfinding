use std::collections::VecDeque;

use bevy::prelude::*;

use crate::tile::VisualTile;

#[derive(Resource, Default)]
pub struct SolveBuffer(VecDeque<Vec<TileColorEvent>>);
impl SolveBuffer {
    pub fn process_frame(&mut self, sprite_query: &mut Query<&mut Sprite, With<VisualTile>>) {
        if let Some(event_vec) = self.0.pop_front() {
            for event in event_vec {event.apply(sprite_query)}
        }
    }
}

#[derive(Debug)]
pub struct TileColorEvent {
    pub sprite_entity: Entity,
    pub color: Color
}
impl TileColorEvent {
    pub fn apply(self, sprite_query: &mut Query<&mut Sprite, With<VisualTile>>) {
        sprite_query.get_mut(self.sprite_entity).unwrap().color = self.color;
    }
}


#[derive(Resource)]
pub struct UpdateTimer(pub Timer);
impl Default for UpdateTimer {
    fn default() -> Self {
        Self(Timer::new(std::time::Duration::from_millis(10), TimerMode::Repeating))
    }
}


pub fn process_update_buffer_system(
    mut timer: ResMut<UpdateTimer>,
    time: Res<Time>,
    mut solve_buffer: ResMut<SolveBuffer>,
    mut sprite_query: Query<&mut Sprite, With<VisualTile>>
) {
    timer.0.tick(time.delta());

    for _ in 0..timer.0.times_finished_this_tick() {
        solve_buffer.process_frame(&mut sprite_query);
    }
}
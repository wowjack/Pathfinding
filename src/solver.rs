use bevy::prelude::*;

#[derive(Default)]
enum Algorithm {
    #[default]
    AStar
}
#[derive(Default)]
enum Heuristic {
    #[default]
    Euclidean
}

#[derive(Default, Resource)]
pub struct SolverState {
    algorithm: Algorithm,
    heuristic: Heuristic
}

use bevy::prelude::*;

use crate::solve_buffer::SolveBuffer;

#[derive(Default, Resource)]
pub struct SolverState {
    algorithm: Algorithm,
    heuristic: Heuristic
}


#[derive(Default)]
enum Algorithm {
    #[default]
    AStar
}
impl Algorithm {
    pub fn get_algorithm(&self) -> fn(SolveBuffer, heuristic: fn((usize, usize), (usize, usize)) -> f32) {
        match self {
            Self::AStar => a_star,
        }
    }
}

fn a_star(
    solve_buffer: SolveBuffer,
    heuristic: fn((usize, usize), (usize, usize)) -> f32
) {
    
}








#[derive(Default)]
enum Heuristic {
    #[default]
    Euclidean
}
impl Heuristic {
    pub fn get_heuristic(&self) -> fn((usize, usize), (usize, usize)) -> f32{
        match self {
            Self::Euclidean => euclidean_heuristic,
        }
    }
}

fn euclidean_heuristic((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> f32 {
    ((ax as f32 - bx as f32).powi(2) + (ay as f32 - by as f32).powi(2)).sqrt()
}

use bevy::prelude::*;

use crate::{solve_buffer::{SolveBuffer, TileColorEvent}, grid::Grid, tile::{CLOSED_COLOR, Tile, TileType, PATH_COLOR, OPEN_COLOR}};

#[derive(Default, Resource)]
pub struct SolverState {
    pub algorithm: Algorithm,
    pub heuristic: Heuristic
}


#[derive(Default)]
pub enum Algorithm {
    #[default]
    AStar
}
impl Algorithm {
    pub fn get_algorithm(&self) -> fn(&mut Grid, &mut SolveBuffer, heuristic: fn((usize, usize), (usize, usize)) -> f32) {
        match self {
            Self::AStar => a_star,
        }
    }
}

#[derive(Clone)]
struct ListItem {
    pub tile: Tile, //Entity referencing visual tile
    pub d: f32, //computed distance from start to tile
    pub h: f32 //heuristic evaluation of distance from tile to end
}
impl ListItem {
    pub fn new(tile: Tile, d: f32, h: f32) -> Self {
        Self {tile, d, h}
    }
}

fn a_star(
    grid: &mut Grid,
    solve_buffer: &mut SolveBuffer,
    heuristic: fn((usize, usize), (usize, usize)) -> f32
) {
    let mut open_list = vec![ListItem::new(grid.get_start(), 0., 0.)];
    let mut closed_list: Vec<ListItem> = Vec::new();

    while !open_list.is_empty() {
        //List to store all tile color changes for this iteration
        let mut event_list: Vec<TileColorEvent> = Vec::new();

        //a) find the tile with the least f in the open list
        let (index, _) = open_list.iter().enumerate().min_by(|a, b| if a.1.d+a.1.h < b.1.d+b.1.h {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater}).unwrap();

        //b) pop the tile off the open list
        let tile = open_list.remove(index);

        //c) for each neighbor
        for x in -1..=1 {
            for y in -1..=1 {            
                if (x==0 && y==0) || tile.tile.position.0 as i32+x<0 || tile.tile.position.0 as i32+x>=grid.grid_size as i32 || tile.tile.position.1 as i32+y<0 || tile.tile.position.1 as i32+y>=grid.grid_size as i32 {continue}
                let neighbor = grid.grid[(tile.tile.position.1 as i32+y) as usize][(tile.tile.position.0 as i32+x) as usize];
                match neighbor.tile_type {
                    //1) if the neighbor is the end tile, stop search and build shortest path
                    TileType::End => {
                        solve_buffer.0.push_back(event_list);
                        let mut p = tile.tile.position;
                        let mut t = grid.grid[p.1][p.0];
                        loop {
                            if let TileType::Start = t.tile_type {break}
                            solve_buffer.0.push_back(vec![TileColorEvent::new(t.entity, PATH_COLOR)]);
                            p = t.parent.expect(&format!("Tile {:?} has no parent.", t.position));
                            t = grid.grid[p.1][p.0];
                        }
                        return;
                    },
                    //2) compute d, h, and f for the neighbor node
                    TileType::None => {
                        let d = tile.d + if x.abs()>0 && y.abs()>0 {std::f32::consts::SQRT_2} else {1.};
                        let h = heuristic(neighbor.position, grid.end);

                        //3) if the tile  already exists in closed_list, skip
                        let mut in_closed = false;
                        for check_tile in closed_list.iter_mut() {
                            if neighbor.entity == check_tile.tile.entity {
                                in_closed=true;
                                break;
                            }
                        }
                        if in_closed {continue}

                        //4) if the tile already exists in open_list, update the tile's parent and d if necessary, then skip
                        let mut in_open = false;
                        open_list.iter_mut().for_each(|check_tile| {
                            if neighbor.entity == check_tile.tile.entity {
                                if d < check_tile.d {
                                    check_tile.d = d;
                                    check_tile.h = h;
                                    grid.grid[check_tile.tile.position.1][check_tile.tile.position.0].parent = Some(tile.tile.position);
                                }
                                in_open = true;
                                return;
                            }
                        });
                        //5) otherwise, add the tile to the open list
                        if !in_open {
                            grid.grid[(tile.tile.position.1 as i32+y) as usize][(tile.tile.position.0 as i32+x) as usize].parent = Some(tile.tile.position);
                            open_list.push(ListItem::new(neighbor, d, h));
                            event_list.push(TileColorEvent::new(neighbor.entity, OPEN_COLOR));
                        }
                    },
                    _ => continue,
                }
            }
        }
        //d) add the tile to the closed list 
        closed_list.push(tile.clone());
        if let TileType::None = tile.tile.tile_type {
            event_list.push(TileColorEvent::new(tile.tile.entity, CLOSED_COLOR));
        };
        solve_buffer.0.push_back(event_list);
       
    }
}








#[derive(Default)]
pub enum Heuristic {
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

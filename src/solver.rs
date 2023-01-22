use std::usize;

use bevy::prelude::*;

use crate::{grid_update::{SlowTileUpdateBuffer, SlowTileEvent}, TileRef, GridState, TileType, GRID_SIZE};

#[derive(Resource, Default)]
pub enum Algorithm {
    #[default]
    AStar,
    Dijkstras,
}

pub fn start_solve(alg: &Res<Algorithm>, buffer: &mut SlowTileUpdateBuffer, game: &mut GridState){
    get_alg(&alg)(buffer, game);
}

pub fn get_alg(alg: &Algorithm) ->  fn(&mut SlowTileUpdateBuffer, &mut GridState) {
    match alg {
        Algorithm::AStar => a_star,
        Algorithm::Dijkstras => dijkstras,
    }
}

fn a_star(update_buffer: &mut SlowTileUpdateBuffer, game_state: &mut GridState) {
    //d = distance from start to tile
    //h = estimated movement cost from tile to end. Read about different metrics
    //f = d + h
    //                        d    h
    #[derive(Clone)]
    struct ListItem(TileRef, f32, f32);
    let mut open_list = vec![ListItem(game_state.grid[1][1].clone(), 0., 0.)];
    let mut closed_list: Vec<ListItem> = Vec::new();

    while !open_list.is_empty() {
        //List to store all tile color changes for this iteration
        let mut event_list: Vec<SlowTileEvent> = Vec::new();

        //a) find the tile with the least f in the open list
        let (index, _) = open_list.iter().enumerate().min_by(|a, b| if a.1.1+a.1.2 < b.1.1+b.1.2 {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater}).unwrap();

        //b) pop the tile off the open list
        let tile = open_list.remove(index);

        //c) for each neighbor
        for i in -1..=1 {
            for j in -1..=1 {            
                if (i==0 && j==0) || tile.0.position.0 as i32+i<0 || tile.0.position.0 as i32+i>=GRID_SIZE as i32 || tile.0.position.1 as i32+j<0 || tile.0.position.1 as i32+j>=GRID_SIZE as i32 {continue}
                let neighbor = game_state.grid[(tile.0.position.0 as i32+i) as usize][(tile.0.position.1 as i32+j) as usize];
                match neighbor.tile_type {
                    //1) if the neighbor is the end tile, stop search and build shortest path
                    TileType::End => {
                        update_buffer.0.push_back(event_list);
                        let mut p = tile.0.position;
                        let mut t = game_state.grid[p.0][p.1];
                        loop {
                            if let TileType::Start = t.tile_type {break}
                            update_buffer.0.push_back(vec![SlowTileEvent(t.entity, Color::hex("ee6c4d").unwrap())]);
                            p = t.parent.expect(&format!("Tile {:?} has no parent.", t.position));
                            t = game_state.grid[p.0][p.1];
                        }
                        return;
                    },
                    //2) compute d, h, and f for the neighbor node
                    TileType::None => {
                        let d = tile.1 + if i.abs()>0 && j.abs()>0 {std::f32::consts::SQRT_2} else {1.};
                        let h = heuristic(neighbor.position, game_state.end);

                        //3) if the tile  already exists in closed_list, skip
                        let mut in_closed = false;
                        for check_tile in closed_list.iter_mut() {
                            if neighbor.entity == check_tile.0.entity {
                                in_closed=true;
                                break;
                            }
                        }
                        if in_closed {continue}

                        //4) if the tile already exists in open_list, update the tile's parent and d if necessary, then skip
                        let mut in_open = false;
                        open_list.iter_mut().for_each(|check_tile| {
                            if neighbor.entity == check_tile.0.entity {
                                if d < check_tile.1 {
                                    check_tile.1 = d;
                                    check_tile.2 = h;
                                    game_state.grid[check_tile.0.position.0][check_tile.0.position.1].parent = Some(tile.0.position);
                                }
                                in_open = true;
                                return;
                            }
                        });
                        //5) otherwise, add the tile to the open list
                        if !in_open {
                            game_state.grid[(tile.0.position.0 as i32+i) as usize][(tile.0.position.1 as i32+j) as usize].parent = Some(tile.0.position);
                            open_list.push(ListItem(neighbor, d, h));
                            event_list.push(SlowTileEvent(neighbor.entity, Color::hex("3d5a80").unwrap()));
                        }
                    },
                    _ => continue,
                }
            }
        }
        //d) add the tile to the closed list 
        closed_list.push(tile.clone());
        if let TileType::None = tile.0.tile_type {
            event_list.push(SlowTileEvent(tile.0.entity, Color::hex("98c1d9").unwrap()));
        };
        update_buffer.0.push_back(event_list);
       
    }
}

fn heuristic((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> f32 {
    return ((ax as f32 - bx as f32).powi(2) + (ay as f32 - by as f32).powi(2)).sqrt();
}

fn dijkstras(update_buffer: &mut SlowTileUpdateBuffer, game_state: &mut GridState) {

}


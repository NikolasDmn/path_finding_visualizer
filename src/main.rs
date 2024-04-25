mod maze;
mod path_finders;


use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use min_max_heap::MinMaxHeap;
use bevy::window::WindowResolution;
use maze::{CellSize, CellState, Maze};
use crate::path_finders::dfs::DFS;
use crate::path_finders::djikstras::Djikstras;
use crate::path_finders::a_star::AStar;
use crate::path_finders::path_finder_interface::PathFinder;

const CELL_SIZE: Vec2 = Vec2::new(20.0, 20.0);


#[derive(Resource)]
struct Solver {
    solver: Box<dyn PathFinder + Sync + Send>
}

fn setup(mut commands: Commands, mut window_query: Query<&mut Window>) {
    println!("Setting up maze");
    commands.spawn(Camera2dBundle::default());
    let maze = maze::create_maze(30,30);
    let cell_size = fit_window_to_maze(&mut window_query, &maze);
    commands.insert_resource(CellSize(cell_size));
    commands.insert_resource(Solver {
        solver: Box::new(DFS::new(&maze))
    });
    commands.insert_resource(maze);
}

fn fit_window_to_maze(window_query: &mut Query<&mut Window>, maze: &Maze) -> usize {
    let window = window_query.single();
    let cell_size = if window.resolution.width() > window.resolution.height() {
        window.resolution.height() as usize / maze.height
    } else {
        window.resolution.width() as usize / maze.width
    };
    
    cell_size
}

fn trace_path(path: Vec<(usize,usize)>, mut maze: ResMut<Maze>) {
    let maze_width = maze.width;
    let start = maze.start;
    let end = maze.end;
    for x in 0..maze.width {
        for y in 0..maze.height {
            if maze.get(x,y) == &CellState::EXPLORED {
                maze.set(x,y, CellState::UNEXPLORED);
            }
        }
    }
    for (x,y) in path.iter() {
        maze.set(*x,*y, CellState::PATH);
    }
    maze.set(start.x as usize, start.y as usize, CellState::START);
    maze.set(end.x as usize, end.y as usize, CellState::END);
}

fn run_solver(mut solver: ResMut<Solver>, mut maze: ResMut<Maze>) {
    if solver.solver.is_solved() {
        return
    }
    solver.solver.iterate(&mut maze);
    if solver.solver.is_solved() {
        println!("Solved! With accuracy: {:02}", solver.solver.get_accuracy(&maze));
        trace_path(solver.solver.get_path(&maze), maze);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maze!".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, maze::initial_maze_render.after(setup))
        .add_systems(Update, run_solver)
        .add_systems(Update, maze::update_maze.after(run_solver))
        .run();
}

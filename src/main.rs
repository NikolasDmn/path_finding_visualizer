mod maze;
mod path_finders;

use bevy::input::mouse::{MouseButtonInput, MouseWheel};
use bevy::prelude::*;
use bevy::ui::ContentSize;
use bevy::window::WindowResolution;
use maze::{render_maze, Cell, CellSize, CellState, Maze};
use crate::path_finders::dfs::DFS;
use crate::path_finders::djikstras::Djikstras;
use crate::path_finders::a_star::AStar;
use crate::path_finders::path_finder_interface::PathFinder;



#[derive(Resource)]
struct Solver {
    solver: Box<dyn PathFinder + Sync + Send>
}

#[derive(Resource)]
struct Controls {
    play: bool,
    maze_size: (usize, usize),
    maze_changes: bool,
}


fn toggle_solve(mut controls: ResMut<Controls>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        controls.play = !controls.play;
    }
    
}


fn reset_maze(
    commands: Commands,
    mut controls: ResMut<Controls>, 
    mut window_query: Query<&mut Window>,
    query: Query<Entity, With<Cell>>,) {

    controls.play = false;
    let maze = maze::create_maze(controls.maze_size.0, controls.maze_size.1);
    let cell_size = fit_window_to_maze(&mut window_query, &maze);
    let solver = Solver {
        solver: Box::new(AStar::new(&maze))
    };

    create_resources(commands, cell_size, solver, maze);
    controls.maze_changes = false;
}


fn maze_change(keyboard_input: Res<ButtonInput<KeyCode>>, controls: Res<Controls>) -> bool {
    keyboard_input.just_pressed(KeyCode::KeyR) || controls.maze_changes
}

fn change_maze_size(mut controls: ResMut<Controls>, keyboard_input: Res<ButtonInput<KeyCode>>){
    if keyboard_input.just_pressed(KeyCode::ArrowDown){
        controls.maze_size = (controls.maze_size.0, controls.maze_size.1+1);
        controls.maze_changes = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowUp) && controls.maze_size.1 > 1 {
        controls.maze_size = (controls.maze_size.0, controls.maze_size.1-1);
        controls.maze_changes = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) && controls.maze_size.0 > 1{
        controls.maze_size = (controls.maze_size.0-1, controls.maze_size.1);
        controls.maze_changes = true;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        controls.maze_size = (controls.maze_size.0+1, controls.maze_size.1);
        controls.maze_changes = true;
    }
}

fn create_resources(mut commands: Commands, cell_size: usize, solver: Solver, maze: Maze) {
    commands.insert_resource(CellSize(cell_size));
    commands.insert_resource(solver);
    commands.insert_resource(maze);

}
fn setup(mut commands: Commands, mut window_query: Query<&mut Window>) {
    let maze = maze::create_maze(30,30);
    let cell_size = fit_window_to_maze(&mut window_query, &maze);
    let solver = Solver {
        solver: Box::new(AStar::new(&maze))
    };
    commands.spawn(Camera2dBundle::default());
    create_resources(commands, cell_size, solver, maze);
}


fn fit_window_to_maze(window_query: &mut Query<&mut Window>, maze: &Maze) -> usize {
    let mut window = window_query.single_mut();
    let cell_size = if window.resolution.width() > window.resolution.height() {
        window.resolution.height() as usize / maze.height
    } else {
        window.resolution.width() as usize / maze.width
    };
    cell_size
}

fn trace_path(path: Vec<(usize,usize)>, mut maze: ResMut<Maze>) {
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
    maze.set(start.0 as usize, start.1 as usize, CellState::START);
    maze.set(end.0 as usize, end.1 as usize, CellState::END);
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

fn should_run_solver(controls: Res<Controls>, solver: Res<Solver>) -> bool {
    controls.play && !solver.solver.is_solved()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maze!".into(),
                resolution: WindowResolution::new(800., 800.),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Controls {
            play: true,
            maze_size: (30,30),
            maze_changes: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, maze::render_maze.after(setup))
        .add_systems(Update, run_solver.run_if(should_run_solver))
        .add_systems(Update, maze::update_maze.after(run_solver).after(render_maze))
        .add_systems(Update, toggle_solve)
        .add_systems(Update, change_maze_size)
        .add_systems(Update, (reset_maze, render_maze.after(reset_maze)).run_if(maze_change))
        .add_event::<MouseWheel>()
        .run();
}

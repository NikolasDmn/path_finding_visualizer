use bevy::ecs::entity::Entity;
use bevy::prelude::{Color, Commands, Component, Query, Res, Resource, Sprite, SpriteBundle, Transform, Window, With};
use bevy::math::Vec2;
use bevy::utils::dbg;
use std::collections::VecDeque;
use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CellState {
    START,
    UNEXPLORED,
    EXPLORED,
    WALL,
    PATH,
    END,
}

#[derive(Component)]
pub struct Cell{
    pub position: (usize, usize),
    pub type_: CellState
}

#[derive(Resource)]
pub struct CellSize(pub usize);

fn get_index(point: (usize,usize), width: usize) -> usize {
    point.1 * width + point.0
}


fn carve_maze(x: usize, y: usize, width: usize, height: usize, maze: &mut Vec<bool>) {
    let mut rng = thread_rng();
    let directions: [(isize, isize); 4] = [(2 , 0), (-2 , 0 ), (0 , 2 ), (0 , -2 )];
    let mut shuffled_directions = directions.to_vec();
    shuffled_directions.shuffle(&mut rng);

    for &(dx, dy) in shuffled_directions.iter() {
        let nx = x as isize + dx as isize;
        let ny = y as isize + dy as isize;

        if nx > 0 && (nx as usize)< width  && ny > 0 && (ny as usize) < height  {
            let nx = nx as usize;
            let ny = ny as usize;
            if maze[ny * width + nx] {
                let mid_x = (x as isize + (dx / 2)) as usize;
                let mid_y = (y as isize + (dy / 2)) as usize;
                maze[mid_y * width + mid_x] = false;
                maze[ny * width + nx] = false;
                carve_maze(nx, ny, width, height, maze);
            }
        }
    }
}

fn get_appropriate_endpoint(maze: &Vec<bool>, width: usize, height: usize, start: (usize,usize)) -> (usize,usize) {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; width * height]; // Visited flag for each cell

    queue.push_back((start, 0)); // (x, y, distance)
    visited[get_index(start, width)] = true;

    let mut farthest_point = start;
    let mut max_distance = 0;

    while let Some((point, dist)) = queue.pop_front() {
        // Update farthest point
        if dist > max_distance {
            max_distance = dist;
            farthest_point = point;
        }
        let directions: Vec<(usize,usize)> = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .map(|&(x,y)| (point.0 as isize + x, point.1 as isize + y))
                        .filter(|&(nx,ny)| nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize)
                        .map(|(nx,ny)| (nx as usize, ny as usize))
                        .filter(|&(nx,ny)| !visited[get_index((nx,ny), width)] && !maze[get_index((nx,ny), width)]).collect::<Vec<(usize,usize)>>();
        // Explore neighbors
        for &new_point in &directions {
            visited[get_index(new_point, width)] = true;
            queue.push_back((new_point, dist + 1));
        }
    }

    farthest_point
}

pub fn create_maze(width: usize, height: usize) -> Maze {
    // First create bitmap of the maze to run the carving algorithm to.
    let mut bit_maze = vec![true; width * height];
    let start_x = rand::random::<usize>() % width;
    let start_y = rand::random::<usize>() % height;
    bit_maze[start_y * width + start_x] = false;
    carve_maze(start_x, start_y, width, height, &mut bit_maze);
    let start = (start_x, start_y);
    let end = get_appropriate_endpoint(&bit_maze, width, height, start);
    let mut cells = bit_maze
                    .into_iter()
                    .map(|cell| if cell{CellState::WALL} else {CellState::UNEXPLORED})
                    .collect::<Vec<CellState>>();
    cells[get_index(start, width)] = CellState::START;
    cells[get_index(end, width)] = CellState::END;
    Maze {
        start,
        end,
        width,
        height,
        cells,
    }
}

#[derive(Resource, Debug)]
pub struct Maze {
    pub(crate) start: (usize,usize),
    pub(crate) end: (usize,usize),
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) cells: Vec<CellState>,
}

impl Maze {
    pub fn get(&self, x: usize, y:usize) -> &CellState {
        if x > self.width && y > self.height {
            panic!("Position: {},{} is invalid.",x,y);

        }
        &self.cells[y*self.width+x]
    }
    pub fn set(&mut self, x: usize, y:usize, state: CellState) {
        if x > self.width && y > self.height {
            panic!("Position: {},{} is invalid.",x,y);

        }
        self.cells[y*self.width+x] = state;
    }
}

fn get_color(cell: &CellState) -> Color {
    match cell {
        CellState::START => Color::GREEN,
        CellState::END => Color::RED,
        CellState::WALL => Color::BLACK,
        CellState::UNEXPLORED => Color::WHITE,
        CellState::EXPLORED => Color::GRAY,
        CellState::PATH => Color::BLUE,
    }
}

pub fn render_maze(
    mut commands: Commands, 
    maze: Res<Maze>, 
    cell_size: Res<CellSize>,
    query: Query<Entity, With<Cell>>, 
    window_query: Query<&Window>) {
    
    for cell in query.iter() {
        commands.entity(cell).remove::<Cell>();
        commands.entity(cell).despawn();
    }
    let window = window_query.single();
    let x_offset = window.resolution.width() / 2.;
    let y_offset = window.resolution.height() / 2.;
    let mut counter = 0;
    for y in 0..maze.height {
        for x in 0..maze.width {
            counter += 1;
            let index = y * maze.width + x;
            let cell = &maze.cells[index];
            commands.spawn(get_tile_sprite(x as f32, x_offset, y as f32, y_offset, cell_size.0 as f32, cell));
        }
    }
}

fn get_tile_sprite(x: f32, x_offset: f32, y: f32, y_offset: f32, cell_size: f32, cell: &CellState) -> (SpriteBundle, Cell){
        let color = get_color(&cell);
        let x_pos = (x * cell_size) - x_offset;
        let y_pos = (y *  cell_size) - y_offset;
        (SpriteBundle {
        transform: Transform::from_xyz(x_pos, y_pos, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(cell_size, cell_size)),
            color,
            ..Default::default()},
        ..Default::default()
        },
        Cell {
        position: (x as usize,y as usize),
        type_: cell.clone()
        }
    )
}
pub fn update_maze(maze: Res<Maze>, mut query:  Query<(&Cell, &mut Sprite)>) {
    for (cell, mut sprite) in query.iter_mut() {

        match cell.type_ {
            CellState::END | CellState::START | CellState::WALL => {

            }
            _ => {
                sprite.color = get_color(&maze.get(cell.position.0 as usize, cell.position.1 as usize));
            }
        }
    }
}



use std::collections::HashMap;
use min_max_heap::MinMaxHeap;
use crate::maze::{CellState, Maze};
use super::path_finder_interface::PathFinder;

pub(crate) struct Djikstras {
    min_heap: MinMaxHeap<(usize, (usize, usize))>,
    distances: HashMap<(usize, usize), usize>,
    solved: bool,
    traversed_cells: usize,
}

impl Djikstras{
    pub fn new(maze: &Maze) -> Self {
        let mut min_heap = MinMaxHeap::new();
        min_heap.push((0, (maze.start.x as usize, maze.start.y as usize)));
        Self {
            min_heap,
            distances: HashMap::new(),
            solved: false,
            traversed_cells: 0,
        }
    }

}

impl PathFinder for Djikstras {
    fn iterate(&mut self, maze: &mut Maze) {
        if self.solved {
            return;
        }
        if self.min_heap.is_empty(){
            dbg!("No path found");
            return;
        }
        let (dist, (x, y)) = self.min_heap.pop_min().unwrap();
        let index = y * maze.width + x;
        if maze.get(x,y) == &CellState::EXPLORED {
            return;
        }
        if maze.get(x,y) == &CellState::END {
            self.solved = true;
            return;
        }
        self.traversed_cells += 1;
        maze.set(x, y, CellState::EXPLORED);
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(dx,dy)| (x as isize + dx,  y as isize + dy))
            .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < maze.width as isize && ny < maze.height as isize)
            .filter(|&(nx, ny)| maze.cells[ny as usize * maze.width + nx as usize] != CellState::WALL)
            .map(|(nx, ny)| (nx as usize, ny as usize)).collect::<Vec<(usize, usize)>>();
        for &(nx, ny) in directions.iter() {
            let next_cost = dist + 1;
            if self.distances.contains_key(&(nx, ny)) && next_cost >= *self.distances.get(&(nx, ny)).unwrap() {
                continue;
            }
            self.distances.insert((nx, ny), next_cost);
            self.min_heap.push((next_cost, (nx, ny)));
        }
    }

    fn get_path(&self, maze: &Maze) -> Vec<(usize, usize)> {
        let maze_width = maze.width;
        let mut step = (maze.end.x as usize, maze.end.y as usize);
        let start = (maze.start.x as usize, maze.start.y as usize);
        let mut path: Vec<(usize,usize)> = vec![];
        while step != start {
            path.push((step.0, step.1));

            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|&(dx, dy)| (step.0 as isize + dx, step.1 as isize + dy))
                .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < maze_width as isize && ny < maze.cells.len() as isize / maze_width as isize) // Ensure in bounds
                .map(|(nx, ny)| (nx as usize, ny as usize))
                .collect::<Vec<(usize, usize)>>();

            let current_distance = self.distances.get(&step).copied().unwrap_or(usize::MAX);

            if let Some(next_step) = directions.iter()
                .filter_map(|&pos| self.distances.get(&pos).map(|&dist| (dist, pos)))
                .filter(|&(dist, _)| dist < current_distance) // Ensure we always move to a lower distance
                .min_by_key(|&(dist, _)| dist) {
                step = next_step.1;
            } else {
                println!("Error: Could not find a valid path back to the start.");
                break; // Break if no valid reduction in path distance is found
            }
        }
        path
    }

    fn is_solved(&self) -> bool {
        self.solved
    }
    
    fn get_accuracy(&self, maze: &Maze) -> f32  {
         self.get_path(maze).len() as f32 / self.traversed_cells as f32
    }
}
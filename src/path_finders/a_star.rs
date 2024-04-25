use min_max_heap::MinMaxHeap;
use std::collections::{HashMap, HashSet};
use crate::maze::{CellState, Maze};

use super::path_finder_interface::PathFinder;

pub struct AStar {
    close_set: HashSet<(usize, usize)>, 
    came_from: HashMap<(usize, usize), (usize, usize)>,
    gscore: HashMap<(usize, usize), usize>,
    fscore: HashMap<(usize, usize), usize>,
    heap: MinMaxHeap<(usize, (usize, usize))>,
    final_coords: Option<(usize, usize)>,
    traversed_cells: usize,

}

impl AStar {
    pub fn new(maze: &crate::maze::Maze) -> Self {
        let mut gscore = HashMap::new();
        gscore.insert((maze.start.0 as usize, maze.start.1 as usize), 0);
        let mut fscore = HashMap::new();
        fscore.insert((maze.start.0 as usize, maze.start.1 as usize), Self::heuristic(maze.start.0 as usize, maze.start.1 as usize, maze.end.0 as usize,maze.end.1 as usize));
        let mut heap = MinMaxHeap::new();
        heap.push((*fscore.get(&(maze.start.0 as usize, maze.start.1 as usize)).unwrap(), (maze.start.0 as usize, maze.start.1 as usize)));
        Self {
            close_set: HashSet::new(),
            came_from: HashMap::new(),
            gscore: gscore,
            fscore: fscore,
            heap,
            final_coords: None,
            traversed_cells: 0,
        }
    }
    fn heuristic(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
        ((x0 as isize - x1 as isize).abs() + (y0 as isize - y1 as isize).abs()) as usize
    }
}


impl PathFinder for AStar {
    fn iterate(&mut self, maze: &mut crate::maze::Maze) {
        if self.heap.is_empty() {
            return;
        }
        let (_, (x, y)) = self.heap.pop_min().unwrap();
        if maze.get(x,y) == &CellState::END {
            self.final_coords = Some((x,y));
            return;
        }
        self.close_set.insert((x,y));
        maze.set(x,y, CellState::EXPLORED);
        self.traversed_cells += 1;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(dx,dy)| (x as isize + dx,  y as isize + dy))
            .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < maze.width as isize && ny < maze.height as isize)
            .map(|(nx, ny)| (nx as usize, ny as usize))
            .filter(|&(nx, ny)| maze.get(nx,ny) != &CellState::WALL)
            .collect::<Vec<(usize, usize)>>();
        
        for &(nx, ny) in directions.iter() {
            let tentative_gscore = *self.gscore.get(&(x,y)).unwrap() + 1;
            if self.gscore.contains_key(&(nx,ny)) &&  tentative_gscore >= *self.gscore.get(&(nx,ny)).unwrap(){
                continue;
            }
            if tentative_gscore < *self.gscore.get(&(nx,ny)).unwrap_or(&usize::MAX) {
                self.came_from.insert((nx,ny), (x,y));
                self.gscore.insert((nx,ny), tentative_gscore);
                self.fscore.insert((nx,ny), tentative_gscore + Self::heuristic(nx,ny, maze.end.0 as usize, maze.end.1 as usize));
                self.heap.push((*self.fscore.get(&(nx,ny)).unwrap(), (nx,ny)));
            }
            
        }
    }

    fn get_path(&self, maze: & Maze) -> Vec<(usize,usize)> {
        let mut current = self.final_coords;

        // Check if there are final coordinates to start from
        if current.is_none() {
            return vec![];  // No final coordinates, not solved
        }

        let mut path = vec![];

        while let Some((x,y)) = current {
            path.push((x,y));
            current = self.came_from.get(&(x,y)).cloned();
        }

        path

    }

    fn is_solved(&self) -> bool {
        self.final_coords.is_some()
    }
    
    fn get_accuracy(&self, maze: &Maze) -> f32  {
         self.get_path(maze).len() as f32 / self.traversed_cells as f32
    }
}
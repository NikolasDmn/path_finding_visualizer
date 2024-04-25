use crate::maze::{CellState, Maze};

use super::path_finder_interface::PathFinder;
use std::collections::{HashSet};

pub(crate) struct DFS {
    stack: Vec<(usize, usize, Vec<(usize,usize)>)>,
    path: Option<Vec<(usize,usize)>>,
    visited: HashSet<(usize,usize)>,
    traversed_cells: usize,
}

impl DFS {
    pub fn new(maze: &Maze) -> Self {
        Self {
            stack: vec![(maze.start.0 as usize, maze.start.1 as usize, vec![(maze.start.0 as usize, maze.start.1 as usize)])],
            path: None,
            visited: HashSet::new(),
            traversed_cells: 0,
        }
    }
}

impl PathFinder for DFS {
    fn iterate(&mut self, maze: &mut crate::maze::Maze) {
        if self.path.is_some() || self.stack.is_empty() {
            return;
        }
        let (x, y, path) = self.stack.pop().unwrap();
        if maze.get(x,y) == &CellState::END {
            self.path = Some(path);
            return;
        }

        maze.set(x,y, CellState::EXPLORED);
        self.visited.insert((x,y));
        self.traversed_cells += 1;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(dx,dy)| (x as isize + dx,  y as isize + dy))
            .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < maze.width as isize && ny < maze.height as isize)
            .map(|(nx, ny)| (nx as usize, ny as usize))
            .filter(|&(nx, ny)| maze.get(nx,ny) != &CellState::WALL)
            .filter(|&(nx, ny)| !self.visited.contains(&(nx,ny)))
            .collect::<Vec<(usize, usize)>>();
        for &(nx, ny) in directions.iter() {
            let mut new_path = path.clone();
            new_path.push((nx, ny));
            self.stack.push((nx, ny, new_path ));
        }
    }

    fn get_path(&self, maze: & crate::maze::Maze) -> Vec<(usize,usize)> {
        if let Some(path) = &self.path {
            return path.clone();
        }
        vec![]
    }

    fn is_solved(&self) -> bool {
        self.path.is_some()
    }
    
    fn get_accuracy(&self, maze: &Maze) -> f32  {
        if let Some(path) = &self.path {
             path.len() as f32 / self.traversed_cells as f32
        } else {
            0.
        }
    }
    
    fn get_new_solver(&mut self, maze: &Maze) -> Box<dyn PathFinder + Sync + Send> {
        Box::new(DFS::new(maze))
    }
}
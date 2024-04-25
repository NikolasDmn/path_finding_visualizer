use crate::Maze;

pub(crate) trait PathFinder: Sync {
    fn iterate(&mut self, maze: &mut Maze);
    fn get_path(&self, maze: & Maze) -> Vec<(usize,usize)>;
    fn is_solved(&self) -> bool;
    fn get_accuracy(&self, maze: &Maze) -> f32 ;
}
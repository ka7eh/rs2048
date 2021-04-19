use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    rng: rand::rngs::ThreadRng,
    grid_size: usize,
    grid: Vec<u64>,
    new_tile_index: Option<usize>,
    score: u64,
}

impl Game {
    pub fn new(grid_size: usize, start_tiles_count: u8) -> Game {
        let grid = vec![0; grid_size * grid_size];

        let mut game = Game {
            rng: thread_rng(),
            grid_size,
            grid,
            new_tile_index: None,
            score: 0,
        };

        for _ in 0..start_tiles_count {
            game.add_tile();
        }

        game.new_tile_index = None;

        game
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        let row = index / self.grid_size;
        let col = index % self.grid_size;
        (row, col)
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        self.grid_size * row + col
    }

    fn add_tile(&mut self) {
        let empty_cells_indices = self.get_empty_cell();
        if empty_cells_indices.len() > 0 {
            let new_tile_index =
                empty_cells_indices[self.rng.gen_range(0..empty_cells_indices.len())];
            let tile_value = if self.rng.gen::<f64>() < 0.9 { 2 } else { 4 };
            self.grid[new_tile_index] = tile_value;
            self.new_tile_index = Some(new_tile_index);
        }
    }

    fn get_empty_cell(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_index, value)| **value == 0)
            .map(|(index, _value)| index)
            .collect()
    }

    fn get_adjacent_tile_index(&self, index: usize, direction: &Direction) -> Option<usize> {
        let (row, col) = self.index_to_coords(index);
        match direction {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some(self.coords_to_index(row - 1, col))
                }
            }
            Direction::Down => {
                if row == self.grid_size - 1 {
                    None
                } else {
                    Some(self.coords_to_index(row + 1, col))
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some(self.coords_to_index(row, col - 1))
                }
            }
            Direction::Right => {
                if col == self.grid_size - 1 {
                    None
                } else {
                    Some(self.coords_to_index(row, col + 1))
                }
            }
        }
    }

    fn move_tile(
        &mut self,
        tile_index: usize,
        direction: &Direction,
        merge_status: &mut Vec<bool>,
    ) -> bool {
        match self.get_adjacent_tile_index(tile_index, direction) {
            Some(adjacent_tile_index) => {
                if self.grid[tile_index] != 0 && !merge_status[adjacent_tile_index] {
                    if self.grid[adjacent_tile_index] == 0 {
                        self.grid[adjacent_tile_index] = self.grid[tile_index];
                        self.grid[tile_index] = 0;
                        self.move_tile(adjacent_tile_index, direction, merge_status);
                        return true;
                    } else if self.grid[tile_index] == self.grid[adjacent_tile_index] {
                        self.grid[adjacent_tile_index] *= 2;
                        self.grid[tile_index] = 0;
                        merge_status[adjacent_tile_index] = true;
                        self.score += self.grid[adjacent_tile_index];
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        let mut merge_status = vec![false; self.grid_size * self.grid_size];
        let mut is_grid_changed = false;
        match direction {
            Direction::Up => {
                for tile_index in 0..self.grid.len() {
                    let is_tile_moved = self.move_tile(tile_index, &direction, &mut merge_status);
                    if !is_grid_changed && is_tile_moved {
                        is_grid_changed = true;
                    }
                }
            }
            Direction::Down => {
                for tile_index in (0..self.grid.len()).rev() {
                    let is_tile_moved = self.move_tile(tile_index, &direction, &mut merge_status);
                    if !is_grid_changed && is_tile_moved {
                        is_grid_changed = true;
                    }
                }
            }
            Direction::Left => {
                for col_index in 0..self.grid_size {
                    for row_index in 0..self.grid_size {
                        let tile_index = self.coords_to_index(row_index, col_index);
                        let is_tile_moved =
                            self.move_tile(tile_index, &direction, &mut merge_status);
                        if !is_grid_changed && is_tile_moved {
                            is_grid_changed = true;
                        }
                    }
                }
            }
            Direction::Right => {
                for col_index in (0..self.grid_size).rev() {
                    for row_index in 0..self.grid_size {
                        let tile_index = self.coords_to_index(row_index, col_index);
                        let is_tile_moved =
                            self.move_tile(tile_index, &direction, &mut merge_status);
                        if !is_grid_changed && is_tile_moved {
                            is_grid_changed = true;
                        }
                    }
                }
            }
        }
        if is_grid_changed {
            self.add_tile();
        }
    }

    pub fn is_finished(&self) -> bool {
        let mut is_finished = true;
        for (tile_index, tile_value) in self.grid.iter().enumerate() {
            if *tile_value == 0 {
                is_finished = false;
                break;
            }
            match self.get_adjacent_tile_index(tile_index, &Direction::Right) {
                Some(right_tile_index) => {
                    if tile_value == &self.grid[right_tile_index] {
                        is_finished = false;
                        break;
                    }
                }
                None => (),
            };
            match self.get_adjacent_tile_index(tile_index, &Direction::Down) {
                Some(down_tile_index) => {
                    if tile_value == &self.grid[down_tile_index] {
                        is_finished = false;
                        break;
                    }
                }
                None => (),
            };
        }
        is_finished
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stats = format!(
            "Score: {: >12} - Finished: {}",
            self.score,
            self.is_finished()
        );
        let length = (self.grid_size * 4) + ((self.grid_size - 1) * 3) + 4;
        let mut lines = vec![stats, "-".repeat(length)];
        let (new_tile_row, new_tile_col) = match self.new_tile_index {
            Some(new_tile_index) => self.index_to_coords(new_tile_index),
            None => (self.grid_size, self.grid_size),
        };
        for row_index in 0..self.grid_size {
            let mut row = Vec::new();
            for col_index in 0..self.grid_size {
                if col_index == 0 {
                    row.push("".to_owned());
                }
                if row_index == new_tile_row && col_index == new_tile_col {
                    row.push(format!(
                        "-{: ^4}-",
                        self.grid[self.coords_to_index(row_index, col_index)]
                    ));
                } else {
                    row.push(format!(
                        " {: ^4} ",
                        self.grid[self.coords_to_index(row_index, col_index)]
                    ));
                }
                if col_index == self.grid_size - 1 {
                    row.push("".to_owned());
                }
            }
            lines.push(row.join("|"));
        }
        lines.push("-".repeat(length));
        write!(f, "{}", lines.join("\r\n"))
    }
}

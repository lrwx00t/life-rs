use core::time;
use std::thread;

use rand::prelude::*;

struct GameOfLife {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    #[inline]
    fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let grid = (0..height)
            .map(|_| (0..width).map(|_| rng.gen::<bool>()).collect())
            .collect();
        Self {
            width,
            height,
            grid,
        }
    }

    #[inline]
    fn next(&mut self) {
        let mut next_grid = vec![vec![false; self.width]; self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let count = self.count_neighbors(i, j);
                next_grid[i][j] = match (self.grid[i][j], count) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.grid = next_grid;
    }

    #[inline]
    fn count_neighbors(&self, i: usize, j: usize) -> usize {
        let mut count = 0;
        // inclusive range operation
        for x in (i.saturating_sub(1))..=(i + 1).min(self.height - 1) {
            for y in (j.saturating_sub(1))..=(j + 1).min(self.width - 1) {
                if (x, y) != (i, j) && self.grid[x][y] {
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    fn print(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                print!("{}", if self.grid[i][j] { "⬜️" } else { "🟥" });
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let ten_millis = time::Duration::from_millis(100);
    let mut game = GameOfLife::new(30, 10);
    for i in 0..2000 {
        print!("\x1B[2J\x1B[1;1H");
        game.print();
        thread::sleep(ten_millis);
        game.next();
    }
}

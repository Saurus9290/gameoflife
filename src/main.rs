use std::collections::HashSet;

type Coordinate = (i32, i32);

struct GameOfLife {
    grid: HashSet<Coordinate>,
}

impl GameOfLife {
    fn new() -> GameOfLife {
        GameOfLife {
            grid: HashSet::new(),
        }
    }

    fn add_cell(&mut self, x: i32, y: i32) {
        self.grid.insert((x, y));
    }

    fn remove_cell(&mut self, x: i32, y: i32) {
        self.grid.remove(&(x, y));
    }

    fn get_neighbors(&self, x: i32, y: i32) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    neighbors.push((x + dx, y + dy));
                }
            }
        }
        neighbors
    }

    fn update(&mut self) {
        let mut new_grid = HashSet::new();
        for &cell in self.grid.iter() {
            let (x, y) = cell;
            let neighbors = self.get_neighbors(x, y);
            let live_neighbors = neighbors.iter().filter(|&n| self.grid.contains(n)).count();
            if live_neighbors == 2 || live_neighbors == 3 {
                new_grid.insert(cell);
            }
            for &neighbor in neighbors.iter() {
                let (nx, ny) = neighbor;
                let neighbor_count = self.get_neighbors(nx, ny).iter().filter(|&n| self.grid.contains(n)).count();
                if neighbor_count == 3 && !self.grid.contains(&neighbor) {
                    new_grid.insert(neighbor);
                }
            }
        }
        self.grid = new_grid;
    }
}
fn main() {
    let mut game = GameOfLife::new();

    // Add some initial cells
    game.add_cell(0, 0);
    game.add_cell(1, 1);
    game.add_cell(2, 0);
    game.add_cell(1, 0);
    game.add_cell(0, 1);

    // Run the simulation
    for _ in 0..10 {
        println!("{:?}", game.grid);
        game.update();
    }
}
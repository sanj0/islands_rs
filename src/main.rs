use std::collections::HashSet;
use std::time::Instant;
use std::thread;

const W: usize = 1000;
const H: usize = 1000;

fn main() {

    let builder = thread::Builder::new()
        .name("island_finder".into())
        .stack_size(16 * 1024 * 1024);

    let mut grid = [false; W * H];
    for i in 0..W * H {
        if rand::random::<f64>() > 0.75 {
            grid[i] = true;
        }
    }

    let handler = builder.spawn(move || {
        let t0 = Instant::now();
        let islands = find_islands(&grid, W as i32, H as i32);
        let dt = t0.elapsed();
        println!("found {} islands in {:.2?} in the {W} x {H} map!", islands.len(), dt);
    }).unwrap();
    handler.join().unwrap();
}

// w, h
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Cell(i32, i32);

// tl, br
#[derive(Copy, Clone, Debug)]
pub struct Island(Cell, Cell);


pub fn find_islands(grid: &[bool], width: i32, height: i32) -> Vec<Island> {
    let mut islands = Vec::<Island>::new();
    let mut visited_cells = HashSet::<Cell>::new();

    for y in 0..height {
        for x in 0..width {
            let cell = Cell(x, y);
            if cell.val(grid, width) && !visited_cells.contains(&cell) {
                islands.push(march_find(grid, width, height, &cell, &mut visited_cells));
            }
        }
    }
    islands
}

fn march_find(grid: &[bool], w: i32, h: i32, cell0: &Cell, visited: &mut HashSet<Cell>) -> Island {
    let mut island = Island(*cell0, *cell0);
    visit_neighbors(grid, w, h, Cell(cell0.0, cell0.1), &mut island, visited);

    island
}

fn visit_neighbors(
    grid: &[bool],
    w: i32,
    h: i32,
    cell: Cell,
    island: &mut Island,
    visited: &mut HashSet<Cell>,
) {
    visited.insert(Cell(cell.0, cell.1));
    let right = cell.0 + 1 < w;
    let left = cell.0 > 0;
    let up = cell.1 > 0;
    let down = cell.1 + 1 < h;

    if right {
        // 1. right
        check_cell(grid, w, h, cell.rel(1, 0), island, visited);
        if up {
            // 2. up right
            check_cell(grid, w, h, cell.rel(1, -1), island, visited);
        }
        if down {
            // 3. down right
            check_cell(grid, w, h, cell.rel(1, 1), island, visited);
        }
    }
    if left {
        // 4. left
        check_cell(grid, w, h, cell.rel(-1, 0), island, visited);
        if up {
            // 5. up left
            check_cell(grid, w, h, cell.rel(-1, -1), island, visited);
        }
        if down {
            // 6. down left
            check_cell(grid, w, h, cell.rel(-1, 1), island, visited);
        }
    }
    if up {
        // 7. up
        check_cell(grid, w, h, cell.rel(0, -1), island, visited);
    }
    if down {
        // 8. up
        check_cell(grid, w, h, cell.rel(0, 1), island, visited);
    }
}

fn check_cell(
    grid: &[bool],
    w: i32,
    h: i32,
    cell: Cell,
    island: &mut Island,
    visited: &mut HashSet<Cell>,
) {
    if cell.val(grid, w) && !visited.contains(&cell) {
        if cell.0 < island.0 .0 {
            island.0 .0 = cell.0;
        } else if cell.0 > island.1 .0 {
            island.1 .0 = cell.0;
        }
        if cell.1 < island.0 .1 {
            island.0 .1 = cell.1;
        } else if cell.1 > island.1 .1 {
            island.1 .1 = cell.1;
        }
        visited.insert(cell);
        visit_neighbors(grid, w, h, cell, island, visited);
    }
}

impl Cell {
    #[inline(always)]
    fn val(&self, grid: &[bool], w: i32) -> bool {
        grid[(self.1 * w + self.0) as usize]
    }

    #[inline(always)]
    fn rel(&self, dx: i32, dy: i32) -> Cell {
        Cell(self.0 + dx, self.1 + dy)
    }
}

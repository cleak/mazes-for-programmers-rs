use ndarray::Array2;
use nalgebra as na;

use rand::Rng;

type Vec2i = na::Vector2<i32>;

// Display table. Index into using this bit order to form the index: (north, east, south, west)
// Combinations with only one direciton are invalid.
static BORDERS: [char; 16] = [
    ' ',
    '║',
    '═',
    '╚',
    '║',
    '║',
    '╔',
    '╠',
    '═',
    '╝',
    '═',
    '╩',
    '╗',
    '╣',
    '╦',
    '╬'
];

// Index bits of each direction
static NORTH: u8 = 0b0001;
static EAST:  u8 = 0b0010;
static SOUTH: u8 = 0b0100;
static WEST:  u8 = 0b1000;

// Cell which contains information about cardinal direction connections.
#[derive(Clone, Copy, Debug)]
struct Cell {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            n: false,
            e: false,
            s: false,
            w: false,
        }
    }
}

// Generate a path using the sidewinder algorithm
fn gen_sidewinder(shape: (usize, usize)) -> Array2<Cell> {
    let mut maze = Array2::<Cell>::default(shape);
    let mut rng = rand::thread_rng();

    for i in (0..shape.0).rev() {
        let mut run_start = 0;

        for j in 0..shape.1 {
            if i == 0 && j == shape.1-1 {
                // Top right, no valid moves
                continue;
            }

            let mut dir = rng.gen::<usize>() % 2;

            if i == 0 {
                dir = 1;
            }

            if j == shape.1-1 {
                dir = 0;
            }

            if dir == 0 {
                let target_j = run_start + rng.gen::<usize>() % (j - run_start + 1);
                run_start = j + 1;

                maze[(i, target_j)].n = true;
                maze[(i-1, target_j)].s = true;
            }

            if dir == 1 {
                maze[(i, j)].e = true;
                maze[(i, j+1)].w = true;
            }
        }
    } 

    maze
}

// Generate a path using the binary tree algorithm
fn gen_btree(shape: (usize, usize)) -> Array2<Cell> {
    let mut maze = Array2::<Cell>::default(shape);

    let mut rng = rand::thread_rng();
    
    for i in (0..shape.0).rev() {
        for j in 0..shape.1 {
            if i == 0 && j == shape.1-1 {
                // Top right, no valid moves
                continue;
            }

            let mut dir = rng.gen::<usize>() % 2;

            if i == 0 {
                dir = 1;
            }

            if j == shape.1-1 {
                dir = 0;
            }

            if dir == 0 {
                maze[(i ,j)].n = true;
                maze[(i-1 ,j)].s = true;
            }

            if dir == 1 {
                maze[(i, j)].e = true;
                maze[(i, j+1)].w = true;
            }
        }
    }

    maze
}

// Display a given maze as ASCII art on stdout
fn display_maze(maze: &Array2<Cell>) {
    let mut display = Array2::<u8>::zeros((maze.dim().0 * 2 + 1, maze.dim().1 * 2 + 1));

    // Help function to assign all the display areas surrounding a cell
    let assign = |dsp: &mut Array2::<u8>, dir: Vec2i, val: u8, idx: Vec2i| {
        let idx1 = idx + dir + Vec2i::new(dir.y, dir.x);
        let idx2 = idx + dir - Vec2i::new(dir.y, dir.x);
        let idx3 = idx + dir;

        let mut val1 = if val == WEST {
            NORTH
        } else {
            val << 1
        };
        let mut val2 = if val == NORTH {
            WEST
        } else {
            val >> 1
        };

        if dir.x == 0 {
            std::mem::swap(&mut val1, &mut val2);
        }
        
        dsp[(idx1.x as usize, idx1.y as usize)] |= val1;
        dsp[(idx2.x as usize, idx2.y as usize)] |= val2;
        dsp[(idx3.x as usize, idx3.y as usize)] |= val1 | val2;
    };

    // Go over each cell
    for i in 0..maze.dim().0 {
        for j in 0..maze.dim().1 {
            let v = Vec2i::new(i as i32 * 2 + 1, j as i32 * 2 + 1);
            let c = &maze[(i, j)];

            // Each blocked edge contributes 1/4 to the display output at a character.
            // Not all combinations are possible since edges are bidirectional.
            if !c.n {
                assign(&mut display, Vec2i::new(-1, 0), NORTH, v);
            }

            if !c.e {
                assign(&mut display, Vec2i::new(0, 1), EAST, v);
            }

            if !c.s {
                assign(&mut display, Vec2i::new(1, 0), SOUTH, v);
            }

            if !c.w {
                assign(&mut display, Vec2i::new(0, -1), WEST, v);
            }

            display[(v.x as usize, v.y as usize)] = 0;
        }
    }

    for i in 0..display.dim().0 {
        for j in 0..display.dim().1 {
            let count = if j % 2 == 1 {
                3
            } else {
                1
            };

            for _ in 0..count {
                print!("{}", BORDERS[display[(i, j)] as usize]);
            }
        }
        println!();
    }
}

fn main() {
    // let maze = gen_btree((12, 12));
    let maze = gen_sidewinder((12, 12));
    display_maze(&maze);
}

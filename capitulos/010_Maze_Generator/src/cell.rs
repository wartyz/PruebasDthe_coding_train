use libfinal::engine::{Canvas, Engine, Window};
use libfinal::matem::random_usize;
use libfinal::color::{fill4, no_stroke, stroke1};
use libfinal::shape::{line, rect};

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    // columna
    pub i: i32,
    // fila
    pub j: i32,
    pub walls: [bool; 4],
    pub visited: bool,
}

pub fn index(i: i32, j: i32, cols: i32, rows: i32) -> Option<i32> {
    if i < 0 || j < 0 || i > cols - 1 || j > rows - 1 {
        return None;
    }
    //println!("En index i + j * cols = {}", i + j * cols);
    Some(i + j * cols)
}

impl Cell {
    pub fn new(i: i32, j: i32) -> Cell {
        Cell {
            i,
            j,
            walls: [true, true, true, true],
            visited: false,
        }
    }

    pub fn check_neighbors(&mut self, cols: i32, rows: i32, grid: &Vec<Cell>) -> Option<usize> {
        let mut neighbors: Vec<usize> = vec![];

        let mut top: Option<usize> = None;
        let mut right: Option<usize> = None;
        let mut bottom: Option<usize> = None;
        let mut left: Option<usize> = None;

        if let Some(i) = index(self.i, self.j - 1, cols, rows) {
            top = Some(i as usize);
        }

        if let Some(i) = index(self.i + 1, self.j, cols, rows) {
            right = Some(i as usize);
        }

        if let Some(i) = index(self.i, self.j + 1, cols, rows) {
            bottom = Some(i as usize);
        }

        if let Some(i) = index(self.i - 1, self.j, cols, rows) {
            left = Some(i as usize);
        }

        if top.is_some() && !grid[top.unwrap()].visited {
            neighbors.push(top.unwrap());
        }
        if bottom.is_some() && !grid[bottom.unwrap()].visited {
            neighbors.push(bottom.unwrap());
        }
        if right.is_some() && !grid[right.unwrap()].visited {
            neighbors.push(right.unwrap());
        }
        if left.is_some() && !grid[left.unwrap()].visited {
            neighbors.push(left.unwrap());
        }

        if neighbors.len() > 0 {
            let r = random_usize(neighbors.len());
            //println!("neighbors[r] = {:?}", neighbors[r]);
            return Some(neighbors[r]);
        } else {
            return None;
        }
    }

    pub fn highlight(&mut self, w: f32, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        let x = (self.i as f32) * w;
        let y = (self.j) as f32 * w;
        no_stroke(&mut engine.param);
        fill4(0.0, 0.0, 255.0, 100.0, &mut engine.param);
        rect(
            //&mut engine.canvas.as_mut().unwrap(),
            canvas,
            &mut engine.param,
            x,
            y,
            w,
            w,
        );
    }

    pub fn show(&mut self, w: f32, engine: &mut Engine, canvas: &mut Canvas<Window>) {
        //let cv = &mut engine.canvas.as_mut().unwrap();
        let pr = &mut engine.param;

        let x = self.i as f32 * w;
        let y = self.j as f32 * w;

        stroke1(255.0, pr);

        if self.walls[0] {
            line(canvas, pr, x, y, x + w, y);
        }
        if self.walls[1] {
            line(canvas, pr, x + w, y, x + w, y + w);
        }
        if self.walls[2] {
            line(canvas, pr, x + w, y + w, x, y + w);
        }
        if self.walls[3] {
            line(canvas, pr, x, y + w, x, y);
        }

        if self.visited {
            fill4(255.0, 0.0, 255.0, 100.0, &mut engine.param);
            rect(
                //&mut engine.canvas.as_mut().unwrap(),
                canvas,
                &mut engine.param,
                x,
                y,
                w,
                w,
            );
        }
    }
}

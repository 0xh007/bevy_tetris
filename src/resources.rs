use bevy::prelude::*;

pub struct TetrisGrid {
    pub grid: [[GridCell; 20]; 10],
}

impl TetrisGrid {
    pub fn construct() -> TetrisGrid {
        let empty_cell = GridCell {
            location: Vec3::new(0.0, 0.0, 0.0),
            occupied: false,
            x: 0,
            y: 0,
        };
        let mut new_grid = [[empty_cell; 20] ; 10];

        let grid_rows = 10 - 1;
        let grid_columns = 20 - 1;
        let x_min = -4.5;
        let x_max = 5.5;
        let y_min = -9.5;
        let y_max = 10.5;
        let static_z = 3.5;


        let mut current_x = x_min;
        for x in 0..grid_rows {
            let mut current_y = y_min;

            for y in 0..grid_columns {
                new_grid[x][y].location = Vec3::new(current_x, current_y, static_z);
                new_grid[x][y].x = x as i32;
                new_grid[x][y].y = y as i32;
                current_y += 1.0;
            }
            current_x += 1.0;
        }

        TetrisGrid { grid: new_grid }
    }

    // Maps the current position of the block to the nearest grid cell
    pub fn update_position(&mut self, position: Vec3) -> (i32, i32) {
        let grid_columns = 10 - 1;
        let grid_rows = 20 - 1;
        let pos_x_round = position.x().round();
        let pos_y_round = position.y().round();
        let mut found_cell = (-1, -1);

        for x in 0..grid_columns {
            for y in 0..grid_rows {
                let grid_x_round = self.grid[x][y].location.x().round();
                let grid_y_round = self.grid[x][y].location.y().round();

                if pos_x_round == grid_x_round {
                    if pos_y_round == grid_y_round {
                        if !self.grid[x][y].occupied  { 
                            self.grid[x][y].occupied = true;
                        }
                            found_cell.0 = x as i32;
                            found_cell.1 = y as i32;

                    }
                    else {
                        if self.grid[x][y].occupied  { 
                            self.grid[x][y].occupied = false;
                        }
                    }
                }
                else {
                    if self.grid[x][y].occupied  { 
                        self.grid[x][y].occupied = false;
                    }
                }
            }
        }

        found_cell
    }

    pub fn is_cell_below_occupied(&self, cur_x: i32, cur_y: i32) -> bool {
        let x = cur_x;
        let y = cur_y - 1;

        // The cell is occupied if we're at the bottom of the grid
        if y < 0 {
            return true;
        } else {
            // Check the cell below if we're not at the bottom
            if cur_y == 1 {
                return true;
            }

            //TODO: Block is stating that it's spot is unoccupied when it's on the floor
            return self.grid[x as usize][y as usize].occupied;
        }

    }

    pub fn print_grid(&self) {
        let grid_rows = 20 - 1;
        println!("-----GRID-----");
        for row in (0..grid_rows).rev() {
            println!("[{}][{}][{}][{}][{}][{}][{}][{}][{}][{}]",
                self.grid[0][row].occupied,
                self.grid[1][row].occupied,
                self.grid[2][row].occupied,
                self.grid[3][row].occupied,
                self.grid[4][row].occupied,
                self.grid[5][row].occupied,
                self.grid[6][row].occupied,
                self.grid[7][row].occupied,
                self.grid[8][row].occupied,
                self.grid[9][row].occupied,
            );
        }
        println!("----END GRID-----");
    }

}

#[derive(Copy, Clone)]
pub struct GridCell {
    pub occupied: bool,
    pub location: Vec3,
    pub x: i32,
    pub y: i32,
}

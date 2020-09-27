use bevy::prelude::*;
use crate::TetronimoState;

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

        let grid_columns = 10;
        let grid_rows = 20;
        let x_min = -4.5;
        let x_max = 5.5;
        let y_min = -9.5;
        let y_max = 10.5;
        let static_z = 3.5;


        let mut current_x = x_min;
        let mut row_count = 0;
        for x in 0..(grid_columns - 1) {
            let mut current_y = y_min;

            for y in 0..(grid_rows - 1) {
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
    pub fn update_position(
        &mut self,
        position: Vec3,
        last_grid_pos: (i32, i32),
        tetronimo_state: TetronimoState,
        ) -> (i32, i32) 
    {
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
                        // Found a matching cell for the moving block
                        
                        // Check the block status and assign the cell to occupied or unoccupied
                        match tetronimo_state {
                            TetronimoState::Stopped => {
                                if !self.grid[x][y].occupied  { 
                                    self.grid[x][y].occupied = true;
                                }
                            },
                            TetronimoState::Moving => {
                                // -1 Is the starting cell position
                                // It gets updated to the proper cell once the blocks start moving
                                if last_grid_pos.0 != -1 && last_grid_pos.1 != -1 {
                                    let unoccupied_x = last_grid_pos.0 as usize;
                                    let unoccupied_y = last_grid_pos.1 as usize;

                                    self.grid[unoccupied_x][unoccupied_y].occupied = false;
                                }
                            },
                        }

                        // Return the matched cell regardless of if we marked it occupied or not
                        found_cell.0 = x as i32;
                        found_cell.1 = y as i32;
                    }
                }
            }
        }

        if found_cell.0 == -1 && found_cell.1 == -1 {
            //println!("Invalid cell");
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

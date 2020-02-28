pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    pub fn new_position(k_row: i32, k_col: i32, q_row: i32, q_col: i32) -> (i32, i32) {
        let new_row = if q_row >= k_row {
            -(q_row - k_row)
        } else {
            k_row - q_row
        };
    
        let new_col = if q_col >= k_col {
            q_col - k_col
        } else {
            -(k_col - q_col)
        };
    
        (new_col, new_row)
    
    }
    
    pub fn near_point(near_val: i32, x_or_y: i32) -> i32 {
        if near_val == -1 {
            return x_or_y.abs();
        } else {
            return std::cmp::min(near_val, x_or_y.abs());
        }
    }
    let mut queens_can_attack = vec![];

    let king_row = king[0];
    let king_col = king[1];

    let mut near_up = -1;
    let mut near_down = -1;

    let mut near_left = -1;
    let mut near_right = -1;

    let mut near_up_left = -1;
    let mut near_up_right = -1;
    let mut near_down_left = -1;
    let mut near_down_right = -1;

    for queen in queens {
        let (queen_col, queen_row) = new_position(king_row, king_col, queen[0], queen[1]);

        if queen_row == 0 {
            if queen_col > 0 {
                if near_left == -1 {
                    near_left = queen_col;
                } else {
                    near_left = std::cmp::min(near_left, queen_col);
                }
            } else {
                if near_right == -1 {
                    near_right = queen_col.abs();
                } else {
                    near_right = std::cmp::min(near_right, queen_col.abs());
                }
            }
        }

        if queen_col == 0 {
            if queen_row > 0 {
               if near_up == -1 {
                   near_up = queen_row;
               } else {
                   near_up = std::cmp::min(near_up, queen_row);
               }
            } else {
                if near_down == -1 {
                    near_down = queen_row.abs();
                } else {
                    near_down = std::cmp::min(near_down, queen_row.abs());
                }
            }
        }

        if queen_row.abs() == queen_col.abs() {
            match (queen_col, queen_row) {
                (x, y) if x > 0 && y > 0 => near_up_left = near_point(near_up_left, x),
                (x, y) if x > 0 && y < 0 => near_down_left = near_point(near_down_left, x),
                (x, y) if x < 0 && y < 0 => near_down_right = near_point(near_down_right, x),
                (x, y) if x < 0 && y > 0 => near_up_right = near_point(near_up_right, x),
                _ => println!("world"),
            }
        }
    }

    println!("{},{},{},{}", near_up, near_down, near_left, near_right);

    if near_up != -1 {
        queens_can_attack.push(vec![king_row - near_up, king_col]);
    }

    if near_down != -1 {
        queens_can_attack.push(vec![king_row + near_down, king_col])
    }

    if near_left != -1 {
        queens_can_attack.push(vec![king_row, king_col + near_left])
    }

    if near_right != -1 {
        queens_can_attack.push(vec![king_row, king_col - near_right])
    }

    if near_up_left != -1 {
        queens_can_attack.push(vec![king_row - near_up_left, king_col + near_up_left]);
    }

    if near_down_left != -1 {
        queens_can_attack.push(vec![king_row + near_down_left, king_col + near_down_left])
    }

    if near_down_right != -1 {
        queens_can_attack.push(vec![king_row + near_down_right, king_col - near_down_right]);
    }

    if near_up_right != -1 {
        queens_can_attack.push(vec![king_row - near_up_right, king_col - near_up_right]);
    }

    queens_can_attack
}

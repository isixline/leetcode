pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut rook_row = 8;
    let mut rook_col = 8;
    
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == 'R' {
                rook_row = i;
                rook_col = j;
                break;
            }
        }
    }

    let mut count = 0;

    for i in rook_row..8 {
        if board[i][rook_col] == 'p' {
            count += 1;
            break;
        }
        if board[i][rook_col] == 'B' {
            break;
        }
    }

    for i in (0..rook_row).rev() {
        if board[i][rook_col] == 'p' {
            count += 1;
            break;
        }
        if board[i][rook_col] == 'B' {
            break;
        }
    }

    for i in rook_col..8 {
        if board[rook_row][i] == 'p' {
            count += 1;
            break;
        }
        if board[rook_row][i] == 'B' {
            break;
        }
    }

    for i in (0..rook_col).rev() {
        if board[rook_row][i] == 'p' {
            count += 1;
            break;
        }
        if board[rook_row][i] == 'B' {
            break;
        }
    }

    count
}

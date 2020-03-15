pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let row = matrix.len();
    let column = matrix[0].len();

    for i in 0..row {
        if i == 0 {
            for j in 0..column {
                let mut di = i;
                let mut dj = j;
                loop {
                    let next_i = di + 1;
                    let next_j = dj + 1;
                    if next_i >= row || next_j >= column {
                        break;
                    }
                    if matrix[di][dj] != matrix[next_i][next_j] {
                        return false;
                    }
                    di = next_i;
                    dj = next_j;
                }
            }
        } else {
            let mut di = i;
            let mut dj = 0;
            loop {
                let next_i = di + 1;
                let next_j = dj + 1;
                if next_i >= row || next_j >= column {
                    break;
                }
                if matrix[di][dj] != matrix[next_i][next_j] {
                    return false;
                }
                di = next_i;
                dj = next_j;
            }
        }
    }
    true
}


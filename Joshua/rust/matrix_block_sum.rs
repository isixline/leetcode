pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let row = mat.len();
    let column = mat[0].len();

    let mut range = vec![vec![0; column + 1]; row + 1];
    let mut answer = vec![vec![0; column]; row];

    for i in 0..row {
        for j in 0..column {
            range[i + 1][j + 1] = range[i + 1][j] + range[i][j + 1] - range[i][j] + mat[i][j];
        }
    }

    for i in 0..row {
        for j in 0..column {
            let r1 = std::cmp::max(0, i as i32 - k) as usize;
            let r2 = std::cmp::min(row as i32, i as i32 + k + 1) as usize;
            let c1 = std::cmp::max(0, j as i32 - k) as usize;
            let c2 = std::cmp::min(column as i32, j as i32 + k + 1) as usize;

            answer[i][j] = range[r2][c2] - range[r2][c1] - range[r1][c2] + range[r1][c1];
        }
    }

    answer 
}

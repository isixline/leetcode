pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = mat.clone();

    let row = mat.len();
    let col = mat[0].len();

    for c in 0..col {
        let mut x = c;
        let mut y = 0;

        let mut v = vec![];
        while x < col && y < row {
            v.push(result[y][x]);
            y += 1;
            x += 1;
        }
        v.sort();
        v.reverse();

        let mut x = c;
        let mut y = 0;

        while x < col && y < row {
            result[y][x] =  v.pop().unwrap();
            y += 1;
            x += 1;
        }
    }

    for r in 0..row {
        let mut x = 0;
        let mut y = r;

        let mut v = vec![];
        while x < col && y < row {
            v.push(result[y][x]);
            y += 1;
            x += 1;
        }
        v.sort();
        v.reverse();

        let mut x = 0;
        let mut y = r;

        while x < col && y < row {
            result[y][x] =  v.pop().unwrap();
            y += 1;
            x += 1;
        }
    }

    result
}

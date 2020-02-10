pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut r = vec![];

    for v in a {
        r.push(flip(reverse(v)));
    }
    r
}

fn reverse(a: Vec<i32>) -> Vec<i32> {
    let mut r = vec![];
    for i in (0..a.len()).rev() {
        r.push(a[i]);
    }
    r
}

fn flip(a: Vec<i32>) -> Vec<i32> {
    let mut r = vec![];

    for x in a {
        if x == 1 {
            r.push(0);
        } else {
            r.push(1);
        }
    }
    r
}

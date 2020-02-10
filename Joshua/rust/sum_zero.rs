pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut v = vec![];

        let serial = n / 2;
        for i in 1..=serial {
            v.push(i);
            v.push(i * -1);
        }
        if n % 2 > 0 {
            v.push(0);
        }
        v
}

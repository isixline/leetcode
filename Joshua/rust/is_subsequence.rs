pub fn is_subsequence(s: String, t: String) -> bool {
    if s.eq(&t) {
        return true;
    }

    let mut idxs: Vec<Vec<usize>> = vec![Vec::new(); s.len()];
    
    let tb = t.as_bytes();
    let sb = s.as_bytes();

    if tb.len() == 0 || sb.len() > tb.len() {
        return false;
    }

    if sb.len() == 0 {
        return true;
    }

    for i in 0..tb.len() {
        for j in 0..sb.len() {
            if sb[j] == tb[i] {
                idxs[j].push(i);
            }
        }
    }

    if idxs[0].len() == 0 {
        return false;
    }

    let mut bound = *idxs[0].iter().min().unwrap();

    let mut matched = true;

    for i in 1..idxs.len() {
        let r = &idxs[i];
        if r.len() == 0 {
            return false;
        }

        if !matched {
            return false;
        }

        for j in 0..r.len() {
            if r[j] > bound {
                bound = r[j];
                matched = true;
                break;
            }
            matched = false;
        }
    }

    matched
}

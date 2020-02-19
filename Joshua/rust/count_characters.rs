pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut cm: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

    for c in chars.chars() {
        cm.entry(c.to_string()).and_modify(|e| {
            *e += 1
        }).or_insert(1);
    }

    let mut matched_words = vec![];

    for w in words {
        let mut cmx: std::collections::HashMap<String, i32> = cm.clone();
        let mut matched = true;

        for c in w.chars() {
            if !cmx.contains_key(&c.to_string()) || *cmx.get(&c.to_string()).unwrap() == 0 {
                matched = false;
                break;
            } else {
                cmx.entry(c.to_string()).and_modify(|e| {
                    *e -= 1
                });
            }
        }
        if matched {
            matched_words.push(w.clone());
        }
    }

    matched_words.iter().map(|e| {
        e.len() as i32
    }).sum()
}

pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut frequence_count = [-1; 26];

        for word in a {
            let mut word_bytes = [0; 26];
            for b in word.bytes() {
                let idx = (b - 97) as usize;
                word_bytes[idx] += 1;
            }

            for i in 0..frequence_count.len() {
                if frequence_count[i] == -1 {
                    frequence_count[i] = word_bytes[i];
                } else {
                    frequence_count[i] = std::cmp::min(frequence_count[i], word_bytes[i]);
                }
            }
        }

        let mut r = vec![];

        for i in 0..frequence_count.len() {
            let f = frequence_count[i];
            for _ in 0..f {
                let c = (i + 97) as u8 as char;
                r.push(c.to_string());
            }
        }
        r
}

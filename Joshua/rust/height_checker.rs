pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut new_heights = heights.clone();
        new_heights.sort();
        
        let mut diff = 0;
        
        for i in 0..heights.len() {
            if heights[i] != new_heights[i] {
                diff += 1;
            }
        }
        diff
}

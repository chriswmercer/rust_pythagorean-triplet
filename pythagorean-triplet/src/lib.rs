use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut return_value:HashSet<[u32; 3]> = HashSet::new();

    if sum >= 6 {
        for a in 1..(sum / 3) {
            for b in (a + 1)..(sum / 2) {
                let c = sum - a - b;
                if a * a + b * b == c * c {
                    return_value.insert([a, b, c]);
                }
            }
        }
    }

    return_value
}

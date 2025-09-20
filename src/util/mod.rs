// stuff that might help me out later in development
pub mod pseudo_moving;
pub fn max_index(evals: &[i64]) -> Option<usize> {
    if evals.is_empty() {
        return None;
    }
    if evals.len() == 1 {
        return Some(0);
    }

    let mut max = evals[0];
    let mut max_index = 0;
    for (index, i) in evals[1..].iter().enumerate() {
        println!("{} {} {}", max, index, i);
        if *i > max {
            max = *i;
            max_index = index;
        }
    }
    return Some(max_index + 1);
}
pub fn min_index(evals: &[i64]) -> Option<usize> {
    if evals.is_empty() {
        return None;
    }
    if evals.len() == 1 {
        return Some(0);
    }

    let mut min = evals[0];
    let mut min_index = 0;
    for (index, i) in evals[1..].iter().enumerate() {
        println!("{} {} {}", min, index, i);
        if *i < min {
            min = *i;
            min_index = index;
        }
    }
    return Some(min_index + 1);
}

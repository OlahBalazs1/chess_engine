use std::time::{Duration, SystemTime};

// stuff that might help me out later in development
pub mod pseudo_moving;
pub fn average_time<F: FnMut() -> ()>(mut op: F, times: u32) -> Duration {
    let mut sum = None;

    for _ in 0..times {
        let start = SystemTime::now();
        op();
        let elapsed = start.elapsed().unwrap();
        if let Some(sum) = sum.as_mut() {
            *sum += elapsed;
        } else {
            sum = Some(elapsed)
        }
    }

    sum.expect("times should be higher than 0") / times
}

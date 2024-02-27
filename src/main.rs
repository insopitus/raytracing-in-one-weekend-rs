use std::{thread, time::Duration};

use lib_rs::{linear_algebra::Vector3,color::Color};
mod image;


fn main() {
    println!("Hello, world!");
    const STEPS: usize = 1000;
    let mut bar = indicatif::ProgressBar::new(STEPS as u64);
    for _ in 0..STEPS {
        thread::sleep(Duration::from_millis(1));
        bar.inc(1);
    }
    bar.finish();
}

// external imports
use indicatif::{ProgressBar, ProgressStyle};

pub mod dem;
pub mod particle_array;


pub fn setup_progress_bar(total_steps: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_steps);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );
    pb
}

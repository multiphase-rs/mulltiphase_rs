// external imports
use indicatif::{ProgressBar, ProgressStyle};

pub mod dem;
pub mod particle_array;
pub mod prelude;


pub fn setup_progress_bar(total_steps: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_steps);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );
    pb
}

/// This trait is implemented, should be implemented by every particle array.
/// So that before any simulation, one can be sure about the particle array
/// attribute lengths and other constants length stuff.
pub trait ValidateParticleArray{
    /// This function makes sure your particle array created for a specific
    /// application is valid and doesn't crate index out of bounds errors
    /// due to accessing and array element by one of the equations
    fn validate_particle_array(&self);
}

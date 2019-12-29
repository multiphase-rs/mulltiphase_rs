// std imports
// these are used to write a file
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


// local crate functions and others
pub use crate::{setup_progress_bar, ValidateParticleArray};
// to write the output
pub use crate::{write_to_vtk, write_nnps_2d_to_vtk};
// to write paraview automation file
pub use crate::paraview::{write_vis_file};

// external crate imports for usage in other crates
pub use cgmath::*;
pub use rayon::prelude::*;
pub use neighbours::prelude::*;
pub use simple_shapes::prelude::*;
pub use indicatif::{ProgressBar, ProgressStyle};

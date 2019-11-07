// use rand::Rng;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// local crate import
use multiphysics::particle_array::Particles;
use multiphysics::setup_progress_bar;
use multiphysics::{euler_step, make_forces_torques_zero, normal_force_dem, write_to_vtk};
use multiphysics::prelude::*;

// external crate imports
use neighbours::nbs2d::NBS2D;
use neighbours::NNPS;
use rayon::prelude::*;

fn main() {
    // create positions inside a box of 2d with dimension 0, 5 in x and 0, 5 in
    // y. With different radius in range 0.1 to 0.3

    // let N = 100;
    // let mut x = vec![];
    // let mut y = vec![];
    // let mut radius = vec![];

    // let mut rng = rand::thread_rng();
    // for _ in 0..N {
    //     x.push(rng.gen_range(0.0, 5.0));
    //     y.push(rng.gen_range(0.0, 5.0));
    //     radius.push(rng.gen_range(0.1, 0.3));
    // }

    let n = 1;
    let x = vec![0.0];
    let y = vec![0.; n];
    let radius = vec![0.5; n];

    let mut pa_left = Particles::from_xyz_rad(&x, &y, &vec![0.; n], &radius);
    pa_left.u[0] = 1.;

    let n = 1;
    let x = vec![1.5];
    let y = vec![0.; n];
    let radius = vec![0.5; n];

    let mut pa_right = Particles::from_xyz_rad(&x, &y, &vec![0.; n], &radius);
    pa_right.u[0] = -1.;

    // ---------------------------------------
    // setup corresponding nbs nnps
    // ----------------------------------------
    let x_min = -1.;
    let x_max = 2.;
    let y_min = -1.;
    let y_max = 2.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d_pa_left = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    nbs2d_pa_left.initialize_next(x.len());
    let mut nbs2d_pa_right = nbs2d_pa_left.clone();
    nbs2d_pa_right.initialize_next(x.len());

    // --------------------------------------

    // solver data
    let dt = 1e-4;
    let mut t = 0.;
    let tf = 1.;
    let mut step_no = 0;
    let pfreq = 100;

    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir_name = project_root.to_owned() + "/two_particles_head_on_two_particle_arrays_output";
    let _p = fs::create_dir(&dir_name);

    // create a progress bar
    let total_steps = (tf / dt) as u64;
    let pb = setup_progress_bar(total_steps);
    while t < tf {
        nbs2d_pa_left.register_particles_to_nnps(&pa_left.x, &pa_left.y, &pa_left.z);
        nbs2d_pa_right.register_particles_to_nnps(&pa_right.x, &pa_right.y, &pa_right.z);

        make_forces_torques_zero!((pa_left, pa_right));
        normal_force_dem!(pa_left, (pa_right), (nbs2d_pa_right));
        normal_force_dem!(pa_right, (pa_left), (nbs2d_pa_left));
        euler_step!((pa_left, pa_right), dt);

        if step_no % pfreq == 0 {
            // println!("{}", step_no);
            write_to_vtk!(
                pa_left,
                format!("{}/particles_left_{}.vtk", &dir_name, step_no)
            );
            write_to_vtk!(
                pa_right,
                format!("{}/particles_right_{}.vtk", &dir_name, step_no)
            );
        }
        step_no += 1;
        t += dt;

        // progress bar increment
        pb.inc(1);
    }
    pb.finish_with_message("Simulation succesfully completed");
}

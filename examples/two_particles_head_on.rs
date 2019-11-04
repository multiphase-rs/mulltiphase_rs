use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

// local crate import
use multiphysics::{make_forces_torques_zero, normal_force_dem, euler_step, write_to_vtk};
use multiphysics::setup_progress_bar;
use multiphysics::particle_array::{Particles};

// external crate imports
use neighbours::nbs2d::NBS2D;
use neighbours::NNPS;
use rayon::prelude::*;


macro_rules! setup_particles_for_head_on_collision{
    ($dest:ident) => {
        $dest.u[0] = 1.;
        $dest.u[1] = -1.;
    };
}

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

    let n = 2;
    let x = vec![0.0, 1.501];
    let y = vec![0.; n];
    let radius = vec![0.5; n];

    let mut pa = Particles::from_xyz_rad(&x, &y, &vec![0.; n], &radius);
    setup_particles_for_head_on_collision!(pa);

    // ---------------------------------------
    // setup corresponding nbs nnps
    // ----------------------------------------
    let x_min = -1.;
    let x_max = 2.;
    let y_min = -1.;
    let y_max = 2.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d_pa = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    nbs2d_pa.initialize_next(x.len());
    // --------------------------------------

    // solver data
    let dt = 1e-4;
    let mut t = 0.;
    let tf = 1.;
    let mut step_no = 0;
    let pfreq = 100;

    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir_name = project_root.to_owned() + "/two_particles_head_on_output";
    let _p = fs::create_dir(&dir_name);

    // create a progress bar
    let total_steps = (tf / dt) as u64;
    let pb = setup_progress_bar(total_steps);
    while t < tf {
        nbs2d_pa.register_particles_to_nnps(&pa.x, &pa.y, &pa.z);

        make_forces_torques_zero!((pa));
        normal_force_dem!(pa, (pa), (nbs2d_pa));
        euler_step!((pa), dt);

        if step_no % pfreq == 0 {
            // println!("{}", step_no);
            write_to_vtk!(pa, format!("{}/particles_{}.vtk", &dir_name, step_no));
        }
        step_no += 1;
        t += dt;

        // progress bar increment
        pb.inc(1);
    }
    pb.finish_with_message("Simulation succesfully completed");

}

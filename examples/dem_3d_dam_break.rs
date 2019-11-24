// std imports
use std::f64::consts::PI;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// local crate import
use multiphysics::particle_array::Particles;
use multiphysics::setup_progress_bar;
use multiphysics::{
    body_force, euler_step, make_forces_torques_zero, normal_force_dem, write_to_vtk,
};
use multiphysics::prelude::*;

// external crate imports
use neighbours::nbs3d::NBS3D;
use neighbours::NNPS;
use rand::Rng;
use simple_shapes::{grid_arange, grid_arange_3d, tank_3d, grid_linspace};
use rayon::prelude::*;


fn main() {
    let x_min = -5.;
    let x_max = 5.;
    let y_min = -5.;
    let y_max = 5.;
    let z_min = -5.;
    let z_max = 5.;
    // create positions inside a box of 2d with dimension 0, 5 in x and 0, 5 in
    // y. With different radius in range 0.1 to 0.3

    let r_min = 0.05;
    let r_max = 0.1;
    let spacing = 2. * r_max;
    let (x, y, z) = grid_arange_3d(
        x_min + 2.,
        x_max - 2.,
        spacing,
        y_min + 2.,
        y_max - 2.,
        spacing,
        z_min + 2.,
        z_max - 2.,
        spacing,
    );
    let n = x.len();
    // let n = 100;
    // let (x, y) = grid_linspace(
    //     x_min + 2.,
    //     x_max - 2.,
    //     n,
    //     y_min + 2.,
    //     y_max - 2.,
    //     n,
    // );
    println!("no of particles {}", x.len());

    let mut rng = rand::thread_rng();
    let rho = 2000.;
    let mut rad_i;
    let mut mass;
    let mut radius = vec![];
    let mut m = vec![];

    for _ in 0..x.len() {
        rad_i = rng.gen_range(r_min, r_max);
        radius.push(rad_i);

        // compute the mass
        mass = PI * rad_i * rad_i * rho;
        m.push(mass);
    }

    let mut sand = Particles::from_xyz_rad(&x, &y, &z, &radius);
    sand.m = m;

    // create wall

    let radius = 0.1;
    let spacing = 2. * radius;
    let (x1, y1, z1) = tank_3d(x_min, x_max, spacing, y_min, y_max, spacing, z_min, z_max, spacing, 2);
    let mut wall = Particles::from_xyz_rad(&x1, &y1, &z1, &vec![radius; x1.len()]);

    // ---------------------------------------
    // setup corresponding nbs nnps
    // ----------------------------------------
    // nbs data structure for neighbour creation
    let max_size = 0.2;
    let mut nbs3d = NBS3D::new(-7., 7., -7., 7., -7., 7., max_size);

    // create nbs for sand
    let mut nbs3d_sand = nbs3d.clone();
    nbs3d_sand.initialize_next(sand.x.len());

    // create nbs for sand
    let mut nbs3d_wall = nbs3d.clone();
    nbs3d_wall.initialize_next(wall.x.len());
    nbs3d_wall.register_particles_to_nnps(&wall.x, &wall.y, &wall.z);

    // --------------------------------------
    // println!("{}", nbs3d_wall.head.len());

    // solver data
    let dt = 1e-4;
    let mut t = 0.;
    let tf = 1.;
    let mut step_no = 0;
    let pfreq = 100;

    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir_name = project_root.to_owned() + "/dem_3d_dam_break_output";
    let _p = fs::create_dir(&dir_name);

    // create a progress bar
    let total_steps = (tf / dt) as u64;
    let pb = setup_progress_bar(total_steps);
    while t < tf {
        nbs3d_sand.register_particles_to_nnps(&sand.x, &sand.y, &sand.z);

        make_forces_torques_zero!((sand));
        body_force!((sand), 0.0, -9.81);
        normal_force_dem!(sand, (sand, wall), (nbs3d_sand, nbs3d_wall));
        euler_step!((sand), dt);

        if step_no % pfreq == 0 {
            write_to_vtk!(sand, format!("{}/sand_{}.vtk", &dir_name, step_no));
            write_to_vtk!(wall, format!("{}/wall_{}.vtk", &dir_name, step_no));
        }
        step_no += 1;
        t += dt;

        // progress bar increment
        pb.inc(1);
    }
    pb.finish_with_message("Simulation succesfully completed");
}

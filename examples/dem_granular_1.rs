// std imports
use std::f32::consts::PI;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// local crate import
use multiphysics::particle_array::Particles;
use multiphysics::setup_progress_bar;
use multiphysics::{
    body_force, euler_step, make_forces_torques_zero, normal_force_dem, write_to_vtk,
};

// external crate imports
use neighbours::nbs2d::NBS2D;
use rand::Rng;
use simple_shapes::{grid_arange, tank_2d, grid_linspace};

fn main() {
    let x_min = -5.;
    let x_max = 5.;
    let y_min = -5.;
    let y_max = 5.;
    // create positions inside a box of 2d with dimension 0, 5 in x and 0, 5 in
    // y. With different radius in range 0.1 to 0.3

    let r_min = 0.05;
    let r_max = 0.07;
    let spacing = 2. * r_max;
    // let (x, y) = grid_arange(
    //     x_min + 2.,
    //     x_max - 2.,
    //     spacing,
    //     y_min + 2.,
    //     y_max - 2.,
    //     spacing,
    // );
    // let n = x.len();
    let n = 100;
    let (x, y) = grid_linspace(
        x_min + 2.,
        x_max - 2.,
        n,
        y_min + 2.,
        y_max - 2.,
        n,
    );
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

    let mut sand = Particles::from_xyz_rad(&x, &y, &vec![0.; n], &radius);
    sand.m = m;

    // create wall

    let radius = 0.1;
    let spacing = 2. * radius;
    let (x1, y1) = tank_2d(x_min, x_max, spacing, y_min, y_max, spacing, 2, true);
    let mut wall = Particles::from_xyz_rad(&x1, &y1, &vec![0.; x1.len()], &vec![radius; x1.len()]);

    // ---------------------------------------
    // setup corresponding nbs nnps
    // ----------------------------------------
    // nbs data structure for neighbour creation
    let max_size = 0.2;
    let mut nbs2d = NBS2D::new(-7., 7., -7., 7., max_size);

    // create nbs for sand
    let mut nbs2d_sand = nbs2d.clone();
    nbs2d_sand.initialize_next(sand.x.len());

    // create nbs for sand
    let mut nbs2d_wall = nbs2d.clone();
    nbs2d_wall.initialize_next(wall.x.len());
    nbs2d_wall.register_particles_to_nbs2d_nnps(&wall.x, &wall.y);

    // --------------------------------------
    // println!("{}", nbs2d_wall.head.len());

    // solver data
    let dt = 1e-4;
    let mut t = 0.;
    let tf = 1.;
    let mut step_no = 0;
    let pfreq = 100;

    let project_root = env!("CARGO_MANIFEST_DIR");
    let dir_name = project_root.to_owned() + "/dem_granular_1_output";
    let _p = fs::create_dir(&dir_name);

    // create a progress bar
    let total_steps = (tf / dt) as u64;
    let pb = setup_progress_bar(total_steps);
    while t < tf {
        nbs2d_sand.register_particles_to_nbs2d_nnps(&sand.x, &sand.y);

        make_forces_torques_zero!((sand));
        body_force!((sand), 0.0, -9.81);
        normal_force_dem!(sand, (sand, wall), (nbs2d_sand, nbs2d_wall));
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

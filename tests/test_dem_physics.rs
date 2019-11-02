// local crate import
use multiphysics::particle_array::Particles;
use multiphysics::setup_progress_bar;
use multiphysics::{euler_step, make_forces_torques_zero, normal_force_dem, write_to_vtk};

// external crate imports
use neighbours::nbs2d::NBS2D;
#[macro_use]
extern crate approx;

#[test]
fn test_force_on_two_particles_overlapping() {
    let n = 2;
    let x = vec![0., 0.8];
    let y = vec![0.; n];
    let radius = vec![0.5; n];

    let mut pa = Particles::from_xyz_rad(&x, &y, &vec![0.; n], &radius);

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

    // forces
    nbs2d_pa.register_particles_to_nbs2d_nnps(&pa.x, &pa.y);
    normal_force_dem!(pa, (pa), (nbs2d_pa));

    // since the overlap amount is
    let overlap = 0.2;
    let force_n = 1e5 * overlap;
    assert_relative_eq!(pa.fx[0], -force_n, epsilon=1.);
    assert_relative_eq!(pa.fx[1], force_n, epsilon=1.);
}
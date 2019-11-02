#[macro_export]
macro_rules! make_forces_torques_zero{
    (($($dest:ident), *)) => {
        $(
            let d_fx = &mut $dest.fx;
            let d_fy = &mut $dest.fy;
            let d_torz = &mut $dest.torz;

            for i in 0..d_fx.len(){
                d_fx[i] = 0.;
                d_fy[i] = 0.;
                d_torz[i] = 0.;
            }
        )*}
}


#[macro_export]
macro_rules! body_force{
    (($($dest:ident), *), $gx: expr, $gy: expr) => {
        $(
            let d_fx = &mut $dest.fx;
            let d_fy = &mut $dest.fy;
            let d_torz = &mut $dest.torz;
            let d_m = &$dest.m;

            for i in 0..d_fx.len(){
                d_fx[i] += d_m[i] * $gx;
                d_fy[i] += d_m[i] * $gy;
            }
        )*}
}

#[macro_export]
macro_rules! normal_force_dem {
    ($dest:ident, ($($sources:ident),*), ($($nnps:ident),*)) => {
        $(
            use rayon::prelude::*;
            let d_radius = &$dest.radius;
            let d_x = &$dest.x;
            let d_y = &$dest.y;
            let d_fx = &mut $dest.fx;
            let d_fy = &mut $dest.fy;
            let d_torz = &mut $dest.torz;

            let s_radius = &$sources.radius;
            let s_x = &$sources.x;
            let s_y = &$sources.y;


            d_fx.par_iter_mut()
                .zip(
                    d_fy.par_iter_mut().zip(
                                    d_torz.par_iter_mut().enumerate()))
                .for_each(|(d_fx_i, (d_fy_i, (i, d_tz_i)))| {

                    let nbrs = $nnps.get_neighbours(d_x[i], d_y[i]);
                    let mut xij_x;
                    let mut xij_y;
                    let mut dist;
                    let mut nij_x;
                    let mut nij_y;
                    let mut overlap;
                    let mut kn;

                    // println!("particle idx {} nbrs {:?}", i, nbrs);
                    for &j in nbrs.iter() {
                        xij_x = s_x[j] - d_x[i];
                        xij_y = s_y[j] - d_y[i];

                        // distance
                        dist = (xij_x.powf(2.) + xij_y.powf(2.)).sqrt();

                        // unit vector passing from d_idx to s_idx
                        if dist > 1e-12 {
                            // println!("{}", dist);
                            nij_x = xij_x / dist;
                            nij_y = xij_y / dist;

                            // find overlap amount
                            overlap = d_radius[i] + s_radius[j] - dist;
                            // println!("distance is {}", dist);
                            // println!("overlap is {}", overlap);
                            kn = 1e5;

                            if overlap > 0. {
                                *d_fx_i -= kn * overlap * nij_x;
                                *d_fy_i -= kn * overlap * nij_y;
                            }

                        }

                    }
                });
        )*}
}

#[macro_export]
macro_rules! euler_step{
    (($($dest:ident), *), $dt: expr) => {
        $(
            let d_x = &mut $dest.x;
            let d_y = &mut $dest.y;
            let d_u = &mut $dest.u;
            let d_v = &mut $dest.v;
            let d_fx = &$dest.fx;
            let d_fy = &$dest.fy;
            let d_m = &$dest.m;

            for i in 0..d_x.len(){
                d_u[i] += d_fx[i] / d_m[i] *$dt;
                d_v[i] += d_fy[i] / d_m[i] *$dt;

                d_x[i] += d_u[i]*$dt;
                d_y[i] += d_v[i]*$dt;
            }
        )*}
}

pub struct Particles {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub m: Vec<f64>,
    pub radius: Vec<f64>,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    pub w: Vec<f64>,
    pub wx: Vec<f64>,
    pub wy: Vec<f64>,
    pub wz: Vec<f64>,
    pub fx: Vec<f64>,
    pub fy: Vec<f64>,
    pub fz: Vec<f64>,
    pub torx: Vec<f64>,
    pub tory: Vec<f64>,
    pub torz: Vec<f64>,
}

impl Particles {
    pub fn new(total_no_particles: usize) -> Self {
        Particles {
            x: vec![0.; total_no_particles],
            y: vec![0.; total_no_particles],
            z: vec![0.; total_no_particles],
            m: vec![0.; total_no_particles],
            radius: vec![0.; total_no_particles],
            u: vec![0.; total_no_particles],
            v: vec![0.; total_no_particles],
            w: vec![0.; total_no_particles],
            wx: vec![0.; total_no_particles],
            wy: vec![0.; total_no_particles],
            wz: vec![0.; total_no_particles],
            fx: vec![0.; total_no_particles],
            fy: vec![0.; total_no_particles],
            fz: vec![0.; total_no_particles],
            torx: vec![0.; total_no_particles],
            tory: vec![0.; total_no_particles],
            torz: vec![0.; total_no_particles],
        }
    }
    pub fn from_xyz_rad(x: &[f64], y: &[f64], z: &[f64], radius: &[f64]) -> Self {
        let total_no_particles = x.len();
        let mut particles = Particles::new(total_no_particles);
        particles.x = x.to_vec();
        particles.y = y.to_vec();
        particles.z = z.to_vec();
        particles.radius = radius.to_vec();

        return particles;
    }
}


#[macro_export]
macro_rules! write_to_vtk {
    ($dest:ident, $filename:expr) => {
        // This is taken from
        // https://lorensen.github.io/VTKExamples/site/VTKFileFormats/#legacy-file-examples
        // let mut filename: String = current_exe().unwrap().to_str().unwrap().to_string();
        // filename.push_str(".vtk");
        let x = &$dest.x;
        let y = &$dest.y;
        let z = &$dest.z;
        let r = &$dest.radius;

        let _ = fs::remove_file($filename);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($filename)
            .unwrap();

        writeln!(file, "# vtk DataFile Version 3.0").unwrap();
        writeln!(file, "Time some").unwrap();
        writeln!(file, "ASCII\nDATASET UNSTRUCTURED_GRID").unwrap();

        writeln!(file, "POINTS {} float", x.len()).unwrap();
        for i in 0..x.len() {
            writeln!(file, "{:.4} {:.4} {:.4}", x[i], y[i], z[i]).unwrap();
        }

        writeln!(file, "POINT_DATA {}", x.len()).unwrap();
        writeln!(file, "SCALARS Diameter float 1").unwrap();
        writeln!(file, "LOOKUP_TABLE default").unwrap();
        for i in 0..x.len() {
            writeln!(file, "{:.4}", r[i]).unwrap();
        }
    };
}

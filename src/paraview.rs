use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_vis_file(filename: String, pa: Vec<&str>, files: Vec<String>, glyph: Vec<bool>, nnps: Vec<&str>, nnps_files: Vec<String>) {
    let _ = fs::remove_file(&filename);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    writeln!(
        file,
        r"
#### import the simple module from the paraview
from paraview.simple import *

# get active view
renderView1 = GetActiveViewOrCreate('RenderView')
"
    )
    .unwrap();

    for i in 0..pa.len() {
        writeln!(
            file,
            "{}_ = LegacyVTKReader(FileNames=[{}])",
            pa[i], files[i]
        )
        .unwrap();

        if glyph[i] == true {
            writeln!(
                file,
                r"
glyph1 = Glyph(Input={}_, GlyphType='Arrow')
glyph1.ScaleArray = ['POINTS', 'Radius']
glyph1.GlyphType = 'Sphere'
glyph1.ScaleFactor = 2.0
glyph1.GlyphMode = 'All Points'

# show data in view
glyph1Display = Show(glyph1, renderView1)

glyph1Display.SetScalarBarVisibility(renderView1, True)
",
                pa[i]
            )
            .unwrap();
        } else {
            writeln!(file, "{}_Display = Show({}_, renderView1)", pa[i], pa[i]).unwrap();
        }
    }

    // write all nnps grid

    for i in 0..nnps.len() {
        writeln!(
            file,
            "{}_ = LegacyVTKReader(FileNames=[{}])",
            nnps[i], nnps_files[i]
        )
        .unwrap();

        writeln!(file, "{}_Display = Show({}_, renderView1)", nnps[i], nnps[i]).unwrap();
    }


    writeln!(
        file,
        r"
renderView1.InteractionMode = '2D'

# reset view to fit data
renderView1.ResetCamera()
# get animation scene
animationScene1 = GetAnimationScene()
animationScene1.UpdateAnimationUsingDataTimeSteps()
# play the animation
animationScene1.Play()
"
    )
    .unwrap();

    // create

    //     $(
    //         writeln!(file, $files.x[0]).unwrap();
    //         writeln!(file, $files).unwrap();
    //     )*
    // }
}

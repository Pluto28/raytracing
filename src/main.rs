use std::io::{self, Write};
pub mod vector;
use vector::Vec3;


fn write_ppm(w: u32, h: u32) {
    // Render 
    print!("P3\n{} {}\n255\n", w, h, );

    for k in 0..h {
        // Progress indicator 
        eprintln!("\rScanlines remaining: {} ", h - k);
        io::stdout().flush().unwrap();
        for j in 0..w {

            let color = Vec3::new(j as f64 / (h - 1) as f64,
                k as f64 / (h - 1) as f64,
                0.0);
            //let r: f64 = j as f64 / (h - 1) as f64;
            //let g: f64 = k as f64 / (h - 1) as f64;
            //let b: f64 = 0.0;

            color.write_color();
            //let ir = (255.0 * r) as u64;
            //let ig = (255.0 * g) as u64;
            //let ib = (255.0 * b) as u64;

            //print!("{} {} {}\n", ir, ig, ib);
       }
    }
}

fn main() {
    // Image

    let height = 256;
    let width  = 256;
    
    let vec_ins = Vec3::new(1.0, 2.0, 4.0);
    write_ppm(width, height);
    eprint!("{} {} {}\n", vec_ins.getx(), vec_ins.gety(), vec_ins.getz());
    eprintln!("\rDone.           \n");
}


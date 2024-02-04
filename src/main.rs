use std::io::{self, Write};

fn main() {
    // Image

    let height = 256;
    let width = 256;
    
    // Render 
    print!("P3\n{} {}\n255\n", width, height, );

    for k in 0..height {
        // Progress indicator 
        eprintln!("\rScanlines remaining: {} ", height - k);
        io::stdout().flush().unwrap();
        for j in 0..width {

            let r: f64 = j as f64 / (width - 1) as f64;
            let g: f64 = k as f64 / (height - 1) as f64;
            let b: f64 = 0.0;

            let ir = (255.0 * r) as u64;
            let ig = (255.0 * g) as u64;
            let ib = (255.0 * b) as u64;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    
    eprintln!("\rDone.           \n");
}

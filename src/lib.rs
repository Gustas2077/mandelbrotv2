use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_mandelbrot(width: usize, height: usize, max_iterations: usize) -> Vec<u32> {
    let mut pixels = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let x0 = (x as f64 / width as f64) * 3.5 - 2.5;
            let y0 = (y as f64 / height as f64) * 2.0 - 1.0;
            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut iteration = 0;

            while x1 * x1 + y1 * y1 <= 4.0 && iteration < max_iterations {
                let xtemp = x1 * x1 - y1 * y1 + x0;
                y1 = 2.0 * x1 * y1 + y0;
                x1 = xtemp;
                iteration += 1;
            }

            let color = if iteration == max_iterations {
                0x000000 // Black for points in the Mandelbrot set
            } else {
                // Map the iteration count to a color (blue to red gradient)
                let r = (iteration as f64 / max_iterations as f64 * 255.0) as u32;
                let g = 0;
                let b = ((max_iterations - iteration) as f64 / max_iterations as f64 * 255.0) as u32;
                (r << 16) | (g << 8) | b
            };

            pixels[y * width + x] = color;
        }
    }

    pixels
}

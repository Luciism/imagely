use imagely::Image;
use std::time::SystemTime;

#[allow(dead_code)]
fn time_benchmark(benchmark: fn() -> (), iterations: usize, label: &str) -> u128 {
    let start_time = SystemTime::now();

    for _ in 0..iterations {
        benchmark();
    }

    let total_time = SystemTime::now().duration_since(start_time).unwrap().as_millis();
    println!(
        "[{}] {} iterations completed with avg of {}ms",
        label, iterations, total_time / iterations as u128);

    total_time
}

#[allow(unused_mut)]
fn main() {
    // Load binary image
    let mut image = Image::load_binary_image("./src/imgs/image.bin", 4, 1080, 1920).unwrap();

    // Or create new image
    // let mut image = Image::new(1000, 1000);

    // Load a secondary image
    let mut image2 = Image::load_binary_image("./src/imgs/image2.bin", 4, 640, 468).unwrap();
    image2.rotate_180();

    // Paste image2 onto image at position (20, 50);
    image.paste(image2, (20, 50));
    image.crop((0, 0, 1080, 1080));

    // Other operations
    image.to_grayscale();
    image.gaussian_blur(3, 5.0);

    // Draw circle
    let color = (255, 0, 0, 255);  // Red
    image.draw_circle_outline((500, 500), 499, color);

    // Save result
    let _ = image.write_binary_image("src/site/media/output.bin");
}
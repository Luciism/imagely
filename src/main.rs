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
    let mut image = Image::new(1000, 1000);
    // let mut image = Image::load_binary_image("./src/imgs/image.bin", 4, 1080, 1920).unwrap();
    // image.draw_circle();
    let color = (0, 255, 0, 255);
    image.draw_circle_outline((500, 500), 499, color);

    // time_benchmark(|| {
    //     // let mut image = Image::load_binary_image("./src/imgs/10x10.bin", 4, 10, 10).unwrap();
    //     let mut image = Image::load_binary_image("./src/imgs/640x468.bin", 4, 640, 468).unwrap();
    //     image.gaussian_blur(3, 5.0);
    // }, iterations, "Gaussian Blur v1");
    //


    // let mut image = Image::load_binary_image("./src/imgs/image.bin", 4, 1080, 1920).unwrap();
    // let mut image = Image::load_binary_image("./src/imgs/640x468.bin", 4, 640, 468).unwrap();
    // let mut image = Image::load_binary_image("./src/imgs/10x10.bin", 4, 10, 10).unwrap();
    // let mut image2 = Image::load_binary_image("./src/imgs/image2.bin", 4, 640, 468).unwrap();
    // // let mut image2 = Image::load_binary_image("./src/imgs/image2.bin", 4, 5, 4).unwrap();
    // image.gaussian_blur(15, 5.0);
    let _ = image.write_binary_image("src/site/media/output.bin");
}

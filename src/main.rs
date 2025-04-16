use std::time;

use imagely::Image;

fn benchmark1() {
    let mut image = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();
    let image2 = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();

    image.paste(image2, (50, 50));
}

fn benchmark2() {
    let mut image = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();
    let image2 = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();

    image.quick_paste(image2, (50, 50));
}

fn run_benchmark() {
    let start = time::SystemTime::now();

    for _ in 0..1000 {
        benchmark1();
    }

    println!("Benchmark 1: {:#?}ms", time::SystemTime::now().duration_since(start).unwrap().as_millis() / 10000);

    // Benchmark 2
    let start = time::SystemTime::now();

    for _ in 0..1000 {
        benchmark2();
    }

    println!("Benchmark 2: {:#?}ms", time::SystemTime::now().duration_since(start).unwrap().as_millis() / 10000);
}


#[allow(unused_mut)]
fn main() {
    run_benchmark();
    //
    // let mut image = Image::load_binary_image("./src/image.bin", 4, 1080, 1920).unwrap();
    // let mut image2 = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();
    // // let mut image2 = Image::load_binary_image("./src/image2.bin", 4, 5, 4).unwrap();
    //
    // image.paste(image2, (50, 50));
    // // image.crop((100, 100, 400, 400));
    // // image.to_grayscale();
    // // image.rotate_90();
    // // image.rotate_270();
    // // image.rotate_180();
    // let _ = image.write_binary_image("src/site/media/output.bin");
}

use imagely::Image;

#[allow(unused_mut)]
fn main() {
    let mut image = Image::load_binary_image("./src/image.bin", 4, 1080, 1920).unwrap();
    let mut image2 = Image::load_binary_image("./src/image2.bin", 4, 640, 468).unwrap();

    image.paste(image2, (300, 250));
    image.crop((100, 100, 400, 400));
    image.to_grayscale();
    image.rotate_90();
    image.rotate_270();
    image.rotate_180();
    let _ = image.write_binary_image("src/site/media/output.bin");
}

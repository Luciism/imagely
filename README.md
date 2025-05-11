# Imagely

### What is Imagely?

A basic proof-of-concept image processing library with a few operations.

*I tried to produce this while doing as little research as possible.*

It is not recommended to actually use this, it was more of just a fun project for myself.

### Functionality

- Load / save binary (RGBA) images
- Conversion between RGB / RGBA
- Pasting images
- Cropping images
- Rotating images (90°, 180°, 270°)
- Convert to black & white
- Apply gaussian blur (extremely slow)
- Draw circle

#### Planned functionality

- Drawing other geomoetric shapes
- Masking
- Utilize GPU

### Website

The website located in `src/site/` is used to convert images to the correct binary format and to preview the binary images.

### Example

```rust
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
```

use image::{open, RgbImage, Rgb, ImageBuffer};
// use image::imageops;

fn main() {

    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];

    println!("{}", img_name);

    let img = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        },
        Ok(i) => /*match*/ i.into_rgb() //{
            // Err(e) => {
            //     println!("Error opening image: {:?}", e);
            //     std::process::exit(1);
            // },
            // Ok(i) => i,
        // }
    };

    let dimensions = match image::image_dimensions(img_name) {
        Err(e) => {
            println!("Error fetching dimensions: {:?}", e);
            std::process::exit(1);
        },
        Ok(d) => d,
    };

    let mut pixels_iter = img.pixels();

    let mut img_mut: RgbImage = RgbImage::new(dimensions.0, dimensions.1);
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let p = pixels_iter.next().unwrap();
            img_mut.put_pixel(x, y, Rgb([p.0[0], p.0[1], p.0[2]]));
        }
    }
   
    img_mut.save("cross.png");
    // let img_pixels: image::Pixels = img.pixels();
    println!("{:?}", dimensions);
}

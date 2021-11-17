use image::{open, RgbImage, Rgb, ImageBuffer, Pixel};
use image::imageops;

trait BoundingBox {
    fn bounding_box(self) -> Self;
}
impl BoundingBox for RgbImage {
    fn bounding_box(self) -> Self {
        // concentrate in average location of all points
        // increase box size until efficiency loss
        self
    }
}

fn main() {

    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];
    let t: i16 = args[2].parse::<i16>().unwrap();
    println!("{}", img_name);

    let (img, dimensions) = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        },
        Ok(i) => (i.into_rgb(), image::image_dimensions(img_name).unwrap())
    };

    // let mut feature_field: RgbImage = detect_features(&img, dimensions, t).unwrap();
    let mut feature_field: RgbImage = detect_features_clean(&img, dimensions, t).unwrap();
    create_feature_image(&feature_field);
    feature_field = feature_field.bounding_box();
    create_final_image(&feature_field);
    // let img_pixels: image::Pixels = img.pixels();
}

fn detect_features_clean(
    img: &RgbImage,
    dimensions: (u32, u32),
    t: i16
) -> std::io::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> { 
    let mut img_mut: RgbImage = RgbImage::new(dimensions.0, dimensions.1);
    let img = imageops::grayscale(img);

    for y in (0+3)..(dimensions.1 - 3) {
        for x in (0 + 3)..(dimensions.0 - 3) {
            let cp = img.get_pixel(x, y).0[0] as i16;
            let mut ps = Vec::with_capacity(16);
            ps.append(&mut vec![
                img.get_pixel(x, y - 3).0[0] as i16,
                img.get_pixel(x + 3, y).0[0] as i16,
                img.get_pixel(x, y + 3).0[0] as i16,
                img.get_pixel(x - 3, y).0[0] as i16,
            ]);
            let mut count = 0;
            for pv in &ps {
                if *pv > cp + t || *pv < cp - t {
                    count += 1;
                }
            }

            let fp: Rgb::<u8> = if count >= 3{
                ps.append(&mut vec![
                    img.get_pixel(x + 1, y - 3).0[0] as i16,
                    img.get_pixel(x + 2, y - 2).0[0] as i16,
                    img.get_pixel(x + 3, y - 1).0[0] as i16,
                    img.get_pixel(x + 3, y + 1).0[0] as i16,
                    img.get_pixel(x + 2, y + 2).0[0] as i16,
                    img.get_pixel(x + 1, y + 3).0[0] as i16,
                    img.get_pixel(x - 1, y + 3).0[0] as i16,
                    img.get_pixel(x - 2, y + 2).0[0] as i16,
                    img.get_pixel(x - 3, y + 1).0[0] as i16,
                    img.get_pixel(x - 3, y - 1).0[0] as i16,
                    img.get_pixel(x - 2, y - 2).0[0] as i16,
                    img.get_pixel(x - 1, y - 3).0[0] as i16,
                ]);
                for pv in ps[4..].iter() {
                    if *pv > cp + t || *pv < cp - t {
                        count += 1;
                    }
                }

                if count > 11 {
                    Rgb([255; 3])
                } else {
                    Rgb([0; 3])
                }
            } else {
                Rgb([0; 3])
            };
            // let fp = if count >= 3 {
            //     Rgb([255, 0, 0])
            // } else {
            //     Rgb([0; 3])
            //     // Rgb([cp as u8; 3])
            // };
            img_mut.put_pixel(x, y, fp);
        }
    }
  
    Ok(img_mut)
}

fn detect_features(
    img: &RgbImage,
    dimensions: (u32, u32),
    t: i16
) -> std::io::Result<ImageBuffer<Rgb<u8>, Vec<u8>>> { 
    let mut img_mut: RgbImage = RgbImage::new(dimensions.0, dimensions.1);
    let img = imageops::grayscale(img);

    for y in (0+3)..(dimensions.1 - 3) {
        for x in (0 + 3)..(dimensions.0 - 3) {
            let cp = img.get_pixel(x, y).0[0] as i16;
            let mut ps = Vec::with_capacity(16);
            ps.append(&mut vec![
                img.get_pixel(x, y - 3).0[0] as i16,
                img.get_pixel(x + 3, y).0[0] as i16,
                img.get_pixel(x, y + 3).0[0] as i16,
                img.get_pixel(x - 3, y).0[0] as i16,
            ]);
            let mut count = 0;
            for pv in &ps {
                if *pv > cp + t || *pv < cp - t {
                    count += 1;
                }
            }

            let fp: Rgb::<u8> = if count < 1 {
                Rgb([cp as u8; 3])
            } else if count < 3{
                Rgb([0, 20, 50])
            } else {
                ps.append(&mut vec![
                    img.get_pixel(x + 1, y - 3).0[0] as i16,
                    img.get_pixel(x + 2, y - 2).0[0] as i16,
                    img.get_pixel(x + 3, y - 1).0[0] as i16,
                    img.get_pixel(x + 3, y + 1).0[0] as i16,
                    img.get_pixel(x + 2, y + 2).0[0] as i16,
                    img.get_pixel(x + 1, y + 3).0[0] as i16,
                    img.get_pixel(x - 1, y + 3).0[0] as i16,
                    img.get_pixel(x - 2, y + 2).0[0] as i16,
                    img.get_pixel(x - 3, y + 1).0[0] as i16,
                    img.get_pixel(x - 3, y - 1).0[0] as i16,
                    img.get_pixel(x - 2, y - 2).0[0] as i16,
                    img.get_pixel(x - 1, y - 3).0[0] as i16,
                ]);
                for pv in ps[4..].iter() {
                    if *pv > cp + t || *pv < cp - t {
                        count += 1;
                    }
                }

                if count > 11 {
                    Rgb([255, 0, 0])
                } else {
                    Rgb([0, 150, 0])
                }
            };
            // let fp = if count >= 3 {
            //     Rgb([255, 0, 0])
            // } else {
            //     Rgb([0; 3])
            //     // Rgb([cp as u8; 3])
            // };
            img_mut.put_pixel(x, y, fp);
        }
    }
  
    Ok(img_mut)
}

fn create_feature_image(img: &RgbImage) -> Result<(), image::ImageError> {
    img.save("cross.png")?;
    Ok(())
}

fn create_final_image(img: &RgbImage) -> Result<(), ()> {
    Ok(())
}

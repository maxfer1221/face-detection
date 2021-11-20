use image::{open, RgbImage, Rgb};
use std::sync::{Arc, Mutex, atomic::{Ordering::SeqCst, AtomicU8}};
use std::path::PathBuf;
use crossbeam::thread as cb_thread;
mod image_processing;
mod features;

// Benchmarking module
use std::time::Instant;

trait Helper where Self: Sized {
    fn feature_image(
        &self, 
        dim: (u32, u32), 
        ff: &features::FeatureField, 
        th: u8, 
        step: usize, 
        img_name: &String) -> Result<(), ()>; 
    // fn bounding_box(self, ff: &FeatureField) -> Self;
}
impl Helper for RgbImage {
    fn feature_image(
        &self, dim: (u32, u32), 
        ff: &features::FeatureField, 
        th: u8, 
        step: usize, 
        img_name: &String) -> Result<(), ()> {

        let img_mut = Arc::new(Mutex::new(RgbImage::new(dim.0, dim.1)));
        let ud = (dim.0 as usize, dim.1 as usize);

        let counter: AtomicU8 = AtomicU8::new(1);

        // let now = Instant::now();
        cb_thread::scope(|s| {
            let mut threads = Vec::new();
            for _i in 0..th {
                threads.push(s.spawn(|_| {
                    let c = counter.fetch_add(1, SeqCst);
                    let bound: usize =
                        (((ud.1 as f64 - 4.0) / th as f64) / step as f64).floor() as usize;
                    for x in (3..(ud.0 - 3)).step_by(step) {
                        for yt in 2..bound {
                            let y = (yt * step * th as usize) + (step * c as usize);
                            if ff.get(x, y) == 1 {
                                let mut img_mut = img_mut.lock().unwrap();
                                (*img_mut).put_pixel(x as u32, y as u32, Rgb([255; 3]));
                            }
                        }
                    }
                }));
            }
            for thread in threads {
                thread.join().unwrap();
            }
        }).unwrap();
        // println!("{:?}", now.elapsed());

        let img_mut = img_mut.lock().unwrap();
        let img_name = PathBuf::from(img_name);
        let img_name = img_name.file_stem().unwrap().to_str().unwrap();
        let now = Instant::now();
        (*img_mut).save(format!("{}{}{}", "out/", img_name, ".png")).unwrap();
        println!("{:?}", now.elapsed());
        Ok(())
    }

    // fn bounding_box(self, ff: &FeatureField) -> Self { 
    //     self
    // }
}

fn main() {

    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];
    let t: i16 = args[2].parse::<i16>().unwrap();
    let th: u8 = args[3].parse::<u8>().unwrap();
    let step: usize = args[4].parse::<usize>().unwrap();

    let (img, dimensions) = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        },
        Ok(i) => (i.into_rgb8(), image::image_dimensions(img_name).unwrap())
    };

    let feature_field = 
        image_processing::detect_features_clean(&img, dimensions, t, th, step).unwrap();
    println!("{:?}", feature_field.find_average((dimensions.0 as usize / 2 as usize, (dimensions.1 / 2) as usize), (300, 300)));
    // img = img.bounding_box(&feature_field);
    let _i = img.feature_image(dimensions, &feature_field, th, step, img_name);
    // create_final_image(&img).unwrap();
}

// fn create_final_image(img: &RgbImage) -> Result<(), ()> {
//     Ok(())
// }

// fn find_average(dim: (u32, u32), ff: &Vec::<Vec::<u8>>) -> (u32, u32) {
//     let Vec::<u32>::with_capacity(dim.0);
//     sums = ff.iter().enumerate().map(|(i, y)| y.iter().enumerate().map(|(j, x)|
//         x * j).sum() * i ).sum();

// }

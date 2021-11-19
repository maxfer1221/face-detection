use image::{open, RgbImage, Rgb, ImageBuffer, Pixel, Luma, Primitive};
use image::imageops;
use std::{time::Instant, sync::{Arc, Mutex, atomic::{Ordering::SeqCst, AtomicU8}}};
use std::thread;
use crossbeam::thread as cb_thread;

struct FeatureField {
    field: Vec<Vec<AtomicU8>>,
    center: (u8, u8),
}

trait Helper where Self: Sized {
    fn feature_image(&self, dim: (u32, u32), ff: &FeatureField, th: u8, step: usize) -> Result<(), ()>; 
    fn bounding_box(self, ff: &FeatureField) -> Self;
}
impl Helper for RgbImage {
    fn feature_image(&self, dim: (u32, u32), ff: &FeatureField, th: u8, step: usize) -> Result<(), ()> {
        let mut img_mut = Arc::new(Mutex::new(RgbImage::new(dim.0, dim.1)));
        let ud = (dim.0 as usize, dim.1 as usize);

        let counter: AtomicU8 = AtomicU8::new(1);

        let now = Instant::now();
        cb_thread::scope(|s| {
            let mut threads = Vec::new();
            for i in 0..th {
                threads.push(s.spawn(|_| {
                    let c = counter.fetch_add(1, SeqCst);
                    let bound: usize =
                        (((ud.1 as f64 - 4.0) / th as f64) / step as f64).floor() as usize;
                    println!("{}", bound);
                    for x in (3..(ud.0 - 3)).step_by(step) {
                        for yt in (2..(bound)) {
                            let y = (yt * step * th as usize) + (step * c as usize);
                            if ff.field[x][y].load(SeqCst) == 1 {
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
        println!("{:?}", now.elapsed());

        let mut img_mut = img_mut.lock().unwrap();
        (*img_mut).save("cross.png").unwrap();
        Ok(())
    }

    fn bounding_box(self, ff: &FeatureField) -> Self { 
        self
    }
}

fn main() {

    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];
    let t: i16 = args[2].parse::<i16>().unwrap();
    let th: u8 = args[3].parse::<u8>().unwrap();
    let step: usize = args[4].parse::<usize>().unwrap();

    let (mut img, dimensions) = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        },
        Ok(i) => (i.into_rgb8(), image::image_dimensions(img_name).unwrap())
    };

    let feature_field = detect_features_clean(&img, dimensions, t, th, step).unwrap();
    img = img.bounding_box(&feature_field);
    let i = img.feature_image(dimensions, &feature_field, th, step);
    create_final_image(&img);
}

fn detect_features_clean(
    img: &RgbImage,
    dim: (u32, u32),
    t: i16,
    th: u8,
    step: usize,
    ) -> std::io::Result<FeatureField> {

    let ud = (dim.0 as usize, dim.1 as usize);
    let img = imageops::grayscale(img);

    let mut fm: Vec<Vec<AtomicU8>> = Vec::with_capacity(ud.0);
    let mut pixels: Vec<Vec<i16>> = Vec::with_capacity(ud.0);


    for x in 0..ud.0 {
        pixels.push(Vec::with_capacity(ud.0));
        for y in 0..ud.1 {
            pixels[x].push(img.get_pixel(x as u32, y as u32).channels()[0] as i16);
        }
    }

    for x in 0..ud.0 {
        fm.push(Vec::with_capacity(ud.1));
        for y in 0..ud.1 {
            fm[x].push(AtomicU8::new(0));
        }
    }
    
    let counter: AtomicU8 = AtomicU8::new(1);
    
    // let now = Instant::now();
    cb_thread::scope(|s| {
        let mut threads = Vec::new();
        for i in 0..th {
            threads.push(s.spawn(|_| {
                let c = counter.fetch_add(1, SeqCst);
                let bound: usize =
                    (((ud.1 as f64 - 4.0) / th as f64) / step as f64).floor() as usize;
                println!("{}", bound);
                for x in (3..(ud.0 - 3)).step_by(step) {
                    for yt in (2..(bound)) {
                        let y = (yt * step * th as usize) + (step * c as usize);
                        if get_feature_value(&pixels, x, y, t) == 1 {
                            fm[x][y].store(1, SeqCst);
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

    Ok(FeatureField { field: fm, center: (0, 0) })
}

fn get_feature_value(pa: &Vec<Vec<i16>>, x: usize, y: usize, t: i16) -> u8 {

    let mut ps = Vec::with_capacity(16);
    let cp = pa[x][y];

    ps.append(&mut vec![
        pa[x][y - 3],
        pa[x + 3][y],
        pa[x][y + 3],
        pa[x - 3][y],
    ]);
    
    let mut count = 0;
    for pv in &ps {
        if *pv > cp + t || *pv < cp - t {
            count += 1;
        }
    }
    
    if count < 3 {
        return 0;
    }

    ps.append(&mut vec![
        pa[x + 1][y - 3],
        pa[x + 2][y - 2],
        pa[x + 3][y - 1],
        pa[x + 3][y + 1],
        pa[x + 2][y + 2],
        pa[x + 1][y + 3],
        pa[x - 1][y + 3],
        pa[x - 2][y + 2],
        pa[x - 3][y + 1],
        pa[x - 3][y - 1],
        pa[x - 2][y - 2],
        pa[x - 1][y - 3],
    ]);
    
    for pv in ps[4..].iter() {
        if *pv > cp + t || *pv < cp - t {
            count += 1;
        }
    }

    if count < 12 {
        return 0;
    }
    
    1
}


fn create_final_image(img: &RgbImage) -> Result<(), ()> {
    Ok(())
}

// fn find_average(dim: (u32, u32), ff: &Vec::<Vec::<u8>>) -> (u32, u32) {
//     let Vec::<u32>::with_capacity(dim.0);
//     sums = ff.iter().enumerate().map(|(i, y)| y.iter().enumerate().map(|(j, x)|
//         x * j).sum() * i ).sum();

// }

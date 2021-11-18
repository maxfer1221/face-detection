use image::{open, RgbImage, Rgb, ImageBuffer, Pixel, Luma, Primitive};
use image::imageops;
use std::{time::Instant, sync::{Arc, Mutex, atomic::{Ordering::SeqCst, AtomicU8}}};
use std::thread;
use crossbeam::thread as cb_thread;

struct FeatureField {
    field: Vec<Vec<AtomicU8>>
}

trait Helper where Self: Sized {
    fn feature_image(&self, dim: (u32, u32), ff: &FeatureField, th: u8) -> Result<(), ()>; 
    fn bounding_box(self, ff: &FeatureField) -> Self;
}
impl Helper for RgbImage {
    fn feature_image(&self, dim: (u32, u32), ff: &FeatureField, th: u8) -> Result<(), ()> {
        let mut img_mut = Arc::new(Mutex::new(RgbImage::new(dim.0, dim.1)));
        let ud = (dim.0 as usize, dim.1 as usize);
        let coords: Arc<Mutex<(u32, u32)>> = Arc::new(Mutex::new((3, 3)));


        cb_thread::scope(|s| {
            let mut threads = Vec::new();
            for i in 0..th {
                threads.push(s.spawn(|_| loop {
                    let mut c = coords.lock().unwrap();
                    let ct = c.clone();
                    if c.0 < dim.0 - 5 {
                        c.0 += 2;
                    } else if c.1 < dim.1 - 5 {
                        c.1 += 2;
                        c.0 = 3;
                    } else {
                        break;
                    }
                    drop(c);
                    // let x = ff.field[ct.0 as usize][ct.1 as usize].lock().unwrap();
                    if ff.field[ct.0 as usize][ct.1 as usize].load(SeqCst) == 1 {
                        let mut img_mut = img_mut.lock().unwrap();
                        (*img_mut).put_pixel(ct.0, ct.1, Rgb([200; 3]));
                    }
                }));
            }
            for thread in threads {
                thread.join().unwrap();
            }
        }).unwrap();
        let t = Instant::now();
        println!("{:?}", t.elapsed());
        let mut img_mut = img_mut.lock().unwrap();
        (*img_mut).save("cross.png").unwrap();
        Ok(())
    }

    fn bounding_box(self, ff: &FeatureField) -> Self { 
        // concentrate in average location of all points
        // increase box size until efficiency loss
        self
    }
}

fn main() {

    let args: &[String] = &std::env::args().collect::<Vec<String>>();
    let img_name: &String = &args[1];
    let t: i16 = args[2].parse::<i16>().unwrap();
    let th: u8 = args[3].parse::<u8>().unwrap();

    let (mut img, dimensions) = match open(img_name) {
        Err(e) => {
            println!("Error opening image: {:?}", e);
            std::process::exit(1);
        },
        Ok(i) => (i.into_rgb8(), image::image_dimensions(img_name).unwrap())
    };

    // let mut feature_field: RgbImage = detect_features(&img, dimensions, t).unwrap();
    let feature_field = detect_features_clean(&img, dimensions, t, th).unwrap();
    img = img.bounding_box(&feature_field);
    let i = img.feature_image(dimensions, &feature_field, th);
    create_final_image(&img);
    // let img_pixels: image::Pixels = img.pixels();
}

fn detect_features_clean(
    img: &RgbImage,
    dim: (u32, u32),
    t: i16,
    th: u8,
    ) -> std::io::Result<FeatureField> {

    let ud = (dim.0 as usize, dim.1 as usize);
    let img = imageops::grayscale(img);

    let coords: Arc<Mutex<(u32, u32)>> = Arc::new(Mutex::new((3, 3)));
    let mut fm: Vec<Vec<AtomicU8>> = Vec::with_capacity(ud.0);
    for y in 0..ud.0 {
        fm.push(Vec::with_capacity(ud.1));
        for x in 0..ud.1 {
            fm[y].push(AtomicU8::new(0));
        }
    }
        // vec![vec![Arc::new(Mutex::new(0u8)); ud.1]; ud.0];

    // let mut counter = (0u64, 0u64);
    // for i in &fm {
    //     for j in  i {
    //         let v = j.lock().unwrap();
    //         if *v == 1 {
    //             counter.0 += 1;
    //         } else {
    //             counter.1 += 1;
    //         }
    //     }
    // }
    // println!("{:?}", counter);
    let counter = Arc::new(Mutex::new((0u64, 0u64)));

    let now = Instant::now();
    cb_thread::scope(|s| {
        let mut threads = Vec::new();
        for i in 0..th {
            threads.push(s.spawn(|_| loop {
                let mut c = coords.lock().unwrap();
                let ct = c.clone();
                if c.0 < dim.0 - 5 {
                    c.0 += 2;
                } else if c.1 < dim.1 - 5 {
                    c.1 += 2;
                    c.0 = 3;
                } else {
                    break;
                }
                drop(c);
                let p_val = get_feature_value(&img, ct.0, ct.1, t);
                let mut counter = counter.lock().unwrap();
                // let mut fm_t = fm[ct.0 as usize][ct.1 as usize].lock().unwrap();
                // println!("{:?}", fm_t);
                if p_val == 1 {
                    fm[ct.0 as usize][ct.1 as usize].store(1, SeqCst);
                    (*counter).0 += 1
                } else {
                    (*counter).1 += 1;
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }).unwrap();

    println!("{:?}", now.elapsed());
    // let mut val = fm[0][0].lock().unwrap();
    // *val = 0;
    // drop(val);
    // let mut counter = (0u64, 0u64);
    // for i in &fm {
    //     for j in  i {
    //         let v = j.lock().unwrap();
    //         if *v == 1 {
    //             counter.0 += 1;
    //         } else {
    //             counter.1 += 1;
    //         }
    //     }
    // }
    // println!("{:?}", counter);

    Ok(FeatureField { field: fm })
}

fn get_feature_value(
    img: &ImageBuffer<impl Pixel<Subpixel = u8> + std::fmt::Debug + 'static, Vec<u8>>, x: u32, y: u32, t: i16) -> u8 {
;
    let mut ps = Vec::with_capacity(16);
    let cp = img.get_pixel(x, y).channels()[0] as i16;

    ps.append(&mut vec![
        img.get_pixel(x, y - 3).channels()[0] as i16,
        img.get_pixel(x + 3, y).channels()[0] as i16,
        img.get_pixel(x, y + 3).channels()[0] as i16,
        img.get_pixel(x - 3, y).channels()[0] as i16,
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
        img.get_pixel(x + 1, y - 3).channels()[0] as i16,
        img.get_pixel(x + 2, y - 2).channels()[0] as i16,
        img.get_pixel(x + 3, y - 1).channels()[0] as i16,
        img.get_pixel(x + 3, y + 1).channels()[0] as i16,
        img.get_pixel(x + 2, y + 2).channels()[0] as i16,
        img.get_pixel(x + 1, y + 3).channels()[0] as i16,
        img.get_pixel(x - 1, y + 3).channels()[0] as i16,
        img.get_pixel(x - 2, y + 2).channels()[0] as i16,
        img.get_pixel(x - 3, y + 1).channels()[0] as i16,
        img.get_pixel(x - 3, y - 1).channels()[0] as i16,
        img.get_pixel(x - 2, y - 2).channels()[0] as i16,
        img.get_pixel(x - 1, y - 3).channels()[0] as i16,
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

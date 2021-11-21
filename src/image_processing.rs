use image::{RgbImage, Pixel, ImageBuffer};
use image::imageops;
use std::sync::{atomic::{Ordering::SeqCst, AtomicU8, AtomicUsize}};
use crossbeam::thread as cb_thread;
use crate::features::FeatureField;

pub fn detect_features_clean(
    img: &RgbImage, 
    dim: (u32, u32), 
    t: i16, 
    th: u8, 
    step: usize) -> std::io::Result<FeatureField> {

    let ud = (dim.0 as usize, dim.1 as usize);
    let img = imageops::grayscale(img);

    let (fm, pixels) = create_matrices(&img, ud);
    let pc = AtomicUsize::new(0);
    
    // let now = Instant::now();
    let counter: AtomicU8 = AtomicU8::new(1);
    cb_thread::scope(|s| {
        let mut threads = Vec::new();
        for _i in 0..th { threads.push(s.spawn(|_| {
            // thread number
            let c = counter.fetch_add(1, SeqCst);

            // scaling thread working rows by thread count and step size
            let bound: usize =
                (((ud.1 as f64 - 4.0) / th as f64) / step as f64).floor() as usize;

            for x in (3..(ud.0 - 3)).step_by(step) {
                for yt in 2..bound {
                    let y = (yt * step * th as usize) + (step * c as usize);
                    if get_feature_value(&pixels, x, y, t) == 1 {
                        pc.fetch_add(1, SeqCst);
                        fm[x][y].store(1, SeqCst);
                    }
                }
            }
        })); }
        // await all threads
        for thread in threads {
            thread.join().unwrap();
        }
    }).unwrap();
    // println!("{:?}", now.elapsed());

    Ok(FeatureField::from(fm, th, (ud.0 as isize, ud.1 as isize), (ud.0/2, ud.1/2), ((ud.0/2) as isize, (ud.1/2) as isize), pc.load(SeqCst)))
}

fn create_matrices<'a>(
    img: &ImageBuffer<impl Pixel<Subpixel = u8> + 'static, Vec<u8>>,
    d: (usize,  usize)) -> (Vec<Vec<AtomicU8>>, Vec<Vec<i16>>) {
    let mut fm: Vec<Vec<AtomicU8>> = Vec::with_capacity(d.0);
    let mut pixels: Vec<Vec<i16>> = Vec::with_capacity(d.0);
    for x in 0..d.0 {
        pixels.push(Vec::with_capacity(d.1));
        fm.push(Vec::with_capacity(d.1));
        for y in 0..d.1 {
            pixels[x].push(img.get_pixel(x as u32, y as u32).channels()[0] as i16);
            fm[x].push(AtomicU8::new(0));
        }
    }

    (fm, pixels)
}

fn get_feature_value(pa: &Vec<Vec<i16>>, x: usize, y: usize, t: i16) -> u8 {
    let mut ps = Vec::with_capacity(16);
    let cp = pa[x][y];

    ps.append(&mut vec![
        pa[x][y - 3], pa[x + 3][y],
        pa[x][y + 3], pa[x - 3][y],
    ]);
    
    let mut count = 0;
    for &pv in &ps {
        if pv > cp + t || pv < cp - t {
            count += 1;
        }
    }
    
    if count < 3 {
        return 0;
    }

    ps.append(&mut vec![
        pa[x + 1][y - 3], pa[x + 2][y - 2], pa[x + 3][y - 1],
        pa[x + 3][y + 1], pa[x + 2][y + 2], pa[x + 1][y + 3],
        pa[x - 1][y + 3], pa[x - 2][y + 2], pa[x - 3][y + 1],
        pa[x - 3][y - 1], pa[x - 2][y - 2], pa[x - 1][y - 3],
    ]);
    
    for &pv in ps[4..].iter() {
        if pv > cp + t || pv < cp - t {
            count += 1;
            if count > 11 {
                return 1;
            }
        }
    }
    0
}

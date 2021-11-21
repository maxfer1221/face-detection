use std::sync::atomic::{Ordering::SeqCst, AtomicU8, AtomicUsize};
use std::cmp::{min, max};
use crossbeam::thread;

pub struct FeatureField {
    field:        Vec<Vec<AtomicU8>>,
    dim:          (isize, isize),
    thread_count: u8,
    bounding_box: BoundingBox,
}

impl FeatureField {
    pub fn from(fm: Vec<Vec<AtomicU8>>, t: u8, d: (isize, isize), c: (usize, usize), r: (isize, isize), pc: usize) -> Self {
        FeatureField {
            field: fm,
            dim: d,
            thread_count: t,
            bounding_box: BoundingBox::from(c, r, pc),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.field[x][y].load(SeqCst)
    }

    pub fn get_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    fn set_box(&mut self, c: (usize, usize), r: (isize, isize), pc: usize) { 
        self.bounding_box = BoundingBox::from(c, r, pc);
    }

    // multithread hopefully :
    fn find_average(&self) -> (usize, usize) {
        let bb = &self.bounding_box;
        let center = (bb.center.0 as isize, bb.center.1 as isize);
        let (sum, pts) = ((AtomicUsize::new(0), AtomicUsize::new(0)), AtomicUsize::new(0));
        for x in max(center.0-bb.range.0, 0)..min(center.0+bb.range.0, self.dim.0) {
            for y in max(center.1-bb.range.1, 0)..min(center.1+bb.range.1, self.dim.1) {
                if self.get(x as usize, y as usize) == 1 {
                    pts.fetch_add(1, SeqCst);
                    sum.0.fetch_add(x as usize, SeqCst); sum.1.fetch_add(y as usize, SeqCst);
                }
            }
        }
        (sum.0.load(SeqCst) / pts.load(SeqCst), sum.1.load(SeqCst) / pts.load(SeqCst))
    }

    pub fn next_box(&mut self, change: (isize, isize)) -> bool {
        // self.bounding_box.center = self.find_average();
        let old_density = self.bounding_box.get_density();
        let density_dif = self.density_difference(change);
        let new_density = density_dif + old_density;

        println!("old: {:?}", old_density);
        println!("new: {:?}", new_density);



        if new_density > old_density {
            let range = (self.get_box().get_range(0) + change.0, self.get_box().get_range(1) + change.1);
            println!("{:?}", range);
            self.bounding_box.range = (self.bounding_box.range.0 + change.0, self.bounding_box.range.1 + change.1);
            let avg = self.find_average();
            println!("{:?}", avg);
            self.bounding_box.center = self.find_average();
            return true;
        }
        false
    }

    fn density_difference(&self, c: (isize, isize)) -> f64 {
        let bb = &self.bounding_box;
        let center = (bb.center.0 as isize, bb.center.1 as isize);
        let bounds = [
            min(center.0 - bb.range.0, center.0 - c.0),
            max(center.0 - bb.range.0, center.0 - c.0),
            min(center.0 + bb.range.0, center.0 + c.0),
            max(center.0 + bb.range.0, center.0 + c.0),
            min(center.1 - bb.range.1, center.1 - c.1),
            max(center.1 - bb.range.1, center.1 - c.1),
            min(center.1 + bb.range.1, center.1 + c.1),
            max(center.1 + bb.range.1, center.1 + c.1),
        ];

        let mut total = 0usize;
        total += 
            self.sum_points((bounds[0], bounds[1]), (bounds[4], bounds[7])) +
            self.sum_points((bounds[2], bounds[3]), (bounds[4], bounds[7])) +
            self.sum_points((bounds[1], bounds[2]), (bounds[4], bounds[5])) +
            self.sum_points((bounds[1], bounds[2]), (bounds[6], bounds[7])) ;

        let area: f64 = ((bb.range.0 as isize + c.0) * (bb.range.1 as isize + c.1)) as f64;
        ((total + bb.get_pixel_count()) as f64 / area) - bb.get_density()
    }
    
    fn sum_points(&self, x: (isize, isize), y: (isize, isize)) -> usize {
        let pts     = AtomicUsize::new(0);
        let counter = AtomicUsize::new(0);
        println!("y before: {:?}", y);

        println!("x: {:?}", x);
        for x in max(x.0, 0)..min(x.1, self.dim.0) {
            for y in max(y.0, 0)..min(y.1, self.dim.1) {
                if self.get(x as usize, y as  usize) == 1{
                    pts.fetch_add(1, SeqCst);
                }
            }
        }
        println!("check ");
        // thread::scope(|s| {
        //     let mut threads = Vec::with_capacity(self.thread_count as usize);
        //     for _i in 0..min(self.thread_count, (y.1 - y.0) as u8) {
        //         threads.push(s.spawn(|_| {
        //             let mut y = y;
        //             let c = counter.fetch_add(1, SeqCst);
        //             y.0 = ((y.1 - y.0) as f64 / self.thread_count as f64).ceil() as isize * c as isize;
        //             y.1 = if (c as isize) < (y.1 - y.0) % self.thread_count as isize {
        //                 ((y.1 - y.0) as f64 / self.thread_count as f64).ceil() as isize + y.0
        //             } else {
        //                 ((y.1 - y.0) as f64 / self.thread_count as f64).floor() as isize + y.0
        //             };
        //             println!("y_c: {:?}, {}", y, c as isize);
        //             for x in x.0..x.1 {
        //                 for y in y.0..y.1 {
        //                     if self.get(x as usize, y as usize) == 1 {
        //                         pts.fetch_add(1, SeqCst);
        //                     }
        //                 }
        //             }
        //         }));
        //     }
        //     for thread in threads {
        //         thread.join().unwrap();
        //     }
        // }).unwrap();
        pts.load(SeqCst)
    }
    

}

pub struct BoundingBox {
    center:      (usize, usize),
    range:       (isize, isize),
    pixel_count: usize,
}

impl BoundingBox {
    fn from(c: (usize, usize), r: (isize, isize), pc: usize) -> Self {
        BoundingBox {
            center: c,
            range: r,
            pixel_count: pc,
        }
    }

    pub fn get_center(&self, i: u8) -> usize {
        match i {
            0 => self.center.0,
            1 => self.center.1,
            _ => {
                panic!("");
            }
        }
    }
    
    pub fn get_range(&self, i: u8) -> isize {
        match i {
            0 => self.range.0,
            1 => self.range.1,
            _ => {
                panic!("");
            }
        }
    }

    fn get_pixel_count(&self) -> usize {
        self.pixel_count
    }

    fn get_area(&self) -> isize {
        self.range.0 * self.range.1 * 4
    }
    
    fn get_density(&self) -> f64 {
        self.pixel_count as f64 / self.get_area() as f64
    }   
}

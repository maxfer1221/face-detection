use std::sync::atomic::{Ordering::SeqCst, AtomicU8};

pub struct FeatureField {
    field:  Vec<Vec<AtomicU8>>,
    center: (u32, u32),
    dim:    (usize, usize),
}

impl FeatureField {
    pub fn from(fm: Vec<Vec<AtomicU8>>, c: (u32, u32), d: (usize, usize)) -> Self {
        FeatureField { field: fm, center: c, dim: d }
    }
    pub fn get_field(&self) -> &Vec<Vec<AtomicU8>> {
        &self.field
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.field[x][y].load(SeqCst)
    }
    pub fn set_center(&mut self, c: (u32, u32)) { 
        self.center = c;
    }
    pub fn find_average(&self, c: (usize, usize), r: (usize, usize)) -> (usize, usize) {
        let mut sum = (0usize, 0usize);
        let mut pts = 0;
        for x in c.0-r.0..c.0+r.0 {
            for y in c.1-r.1..c.1+r.1 {
                if self.get(x, y) == 1 {
                    pts += 1;
                    sum = (sum.0 + x, sum.1 + y);
                }
            }
        }
        (sum.0 / pts, sum.1 / pts)
    }
}

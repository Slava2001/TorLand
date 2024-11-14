use std::usize;

struct Cluster {
    x: usize,
    y: usize,
}

pub struct Voronoi {
    clusters: Vec<Cluster>,
    h: usize,
    w: usize,
}

impl Voronoi {
    fn get_sqdist(p1: (usize, usize), p2: (usize, usize), m: (usize, usize)) -> usize {
        let dx = p1.0.abs_diff(p2.0);
        let dx = dx.min(m.0 - dx);
        let dy = p1.1.abs_diff(p2.1);
        let dy = dy.min(m.1 - dx);
        dx * dx + dy * dy
    }

    pub fn new<R: rand::Rng + ?Sized>(rng: &mut R, h: usize, w: usize, cnt: usize) -> Self {
        let mut clusters = Vec::new();
        for _ in 0..cnt {
            clusters.push(Cluster {
                x: rng.gen_range(0..w),
                y: rng.gen_range(0..h),
            });
        }
        Voronoi { h, w, clusters }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        let mut cluster = 0;
        let mut dist = (self.w + self.h).pow(2);
        for (i, c) in self.clusters.iter().enumerate() {
            let tmp = Self::get_sqdist((x, y), (c.x, c.y), (self.w, self.h));
            if tmp < dist {
                dist = tmp;
                cluster = i;
            }
        }
        cluster
    }
}

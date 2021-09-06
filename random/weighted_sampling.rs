#[derive(Clone)]
pub struct WeightedRandomSampling {
    sz: usize,
    arr: Vec<f64>,
    random: XorShift,
}

impl WeightedRandomSampling {
    pub fn new(n: usize) -> Self {
        let mut sz = 1usize;
        while sz < n {
            sz <<= 1;
        }
        WeightedRandomSampling {
            sz: sz,
            arr: vec![0.0; sz << 1],
            random: XorShift::new(),
        }
    }

    pub fn set(&mut self, idx: usize, w: f64) {
        self.arr[idx + self.sz] = w;
    }

    pub fn build(&mut self) {
        for i in (1..self.sz).rev() {
            self.arr[i] = self.arr[i << 1] + self.arr[(i << 1) | 1];
        }
    }

    pub fn update(&mut self, mut idx: usize, w: f64) {
        idx += self.sz;
        self.arr[idx] = w;
        idx >>= 1;
        while idx > 0 {
            self.arr[idx] = self.arr[idx << 1] + self.arr[(idx << 1) | 1];
            idx >>= 1;
        }
    }

    pub fn sampling(&mut self) -> usize {
        let mut idx = 1;
        while idx < self.sz {
            let left = self.arr[idx << 1];
            let right = self.arr[(idx << 1) | 1];
            let t = self.random.urand();
            idx <<= 1;
            if t >= left / (left + right) {
                idx |= 1;
            }
        }
        idx - self.sz
    }
}

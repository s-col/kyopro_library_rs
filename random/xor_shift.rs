#[derive(Clone)]
pub struct XorShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift {
    pub fn new() -> Self {
        XorShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn urand(&mut self) -> f64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= t ^ (t >> 8) ^ (self.w >> 19);
        self.w as f64 / 4294967296.0
    }

    pub fn randint(&mut self, a: i64, b: i64) -> i64 {
        (self.urand() * ((b - a + 1) as f64)) as i64 + a
    }
}

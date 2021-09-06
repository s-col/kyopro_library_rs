#[derive(Clone)]
pub struct Shuffle {
    random: XorShift,
}

impl Shuffle {
    pub fn new() -> Self {
        Shuffle {
            random: XorShift::new(),
        }
    }

    pub fn shuffle<T: Clone>(&mut self, vec: &mut [T]) {
        let n = vec.len();
        for i in (0..n).rev() {
            let j = self.random.randint(0, i as i64) as usize;
            let tmp = vec[i].clone();
            vec[i] = vec[j].clone();
            vec[j] = tmp;
        }
    }
}

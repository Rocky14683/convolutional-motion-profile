use std::collections::VecDeque;

pub struct MovingAverage {
    queue: VecDeque<f32>,
    size: usize,
    sum: f32,
}


impl MovingAverage {
    pub fn new(size: usize)-> MovingAverage {
        Self {
            queue: Default::default(),
            size,
            sum: 0.0,
        }
    }
    pub fn reset(&mut self) {
        self.sum = 0_f32;
        self.queue.clear();
    }

    pub fn step(&mut self, value: f32) -> f32 {
        if self.queue.len() == self.size {
            self.sum -= self.queue.pop_front().unwrap_or_default();
        }

        self.sum += value;
        self.queue.push_back(value);

        return self.solve();
    }

    pub fn solve(&self) -> f32 {
        if self.queue.is_empty() {
            return 0_f32;
        }

        return self.sum / self.queue.len() as f32;
    }
}
use crate::moving_avg::MovingAverage;

pub struct ConvolutionalMotionProfile {
    distance: f32,
    max_v: f32,
    max_a: f32,
    max_j: f32,
}
impl ConvolutionalMotionProfile {
    pub fn new(distance: f32, max_v: f32, max_a: f32, max_j: f32) -> ConvolutionalMotionProfile {
        return Self {
            distance,
            max_v,
            max_a,
            max_j,
        };
    }

    pub fn generate(&self, dt: f32) -> Vec<f32> {
        // let mut profile: Vec<(f32, f32, f32)> = Vec::new();
        let constraints = [self.max_v, self.max_a, self.max_j];
        let mut profile: Vec<f32> =
            vec![self.max_v; ((self.distance / self.max_v).round() / dt) as usize];

        for i in 0..(constraints.len() - 1) {
            let mut new_profile : Vec<f32> = vec![0_f32; 1];
            let step: u32 = (constraints[i] + constraints[i + 1] / dt) as u32;
            let mut avg = MovingAverage::new(step as usize);
            avg.step(0_f32);

            for speed in profile {
                new_profile.push(avg.step(speed));
            }

            for _i in 0..step {
                new_profile.push(avg.step(0_f32));
            }

            profile = new_profile;
        }

        return profile;
    }
}

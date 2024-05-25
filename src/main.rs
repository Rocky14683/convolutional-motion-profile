mod convolution_mp;
mod moving_avg;
use crate::convolution_mp::ConvolutionalMotionProfile;
use gnuplot::{*};

fn main() {
    const DT: f32 = 0.01;
    let cmp = ConvolutionalMotionProfile::new(10_f32, 1_f32, 5_f32, 20_f32);
    let profile = cmp.generate(DT);


    let mut graph = Figure::new();

    graph.axes2d()
        .set_title("S-Curve Velocity Motion Profile", &[])
        .set_legend(Graph(2.0), Graph(1.0), &[], &[])
        .set_x_label("time in seconds", &[])
        .set_y_label("velocity in m/s", &[])
        .lines(0..(profile.len() * (1.0_f32 / DT) as usize), profile.clone(), &[Caption("Velocity")]);

    graph.show().unwrap();
    //
    // for i in profile {
    //     println!("{}", i);
    // }
}

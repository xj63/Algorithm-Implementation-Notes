use sort::test_data::{gen_function, with_noise};

fn main() {
    const N: usize = 1000;
    let mean = N as f64 / 2.0;
    let std_dev = N as f64 / 6.0;
    let gaussian = |x| {
        let x = x as f64;
        (10000.0 * (-(x - mean).powi(2) / (2.0 * std_dev.powi(2))).exp()) as i32
    };
    let data: [_; N] = with_noise(gen_function(gaussian), -1000..=1000);
    println!("{:?}", data);
}

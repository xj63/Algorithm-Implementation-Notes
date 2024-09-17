use sort::test_data::{gen_function, with_noise};

fn main() {
    const N: usize = 1000;
    let sin = |x| (10000.0 * (x as f64 / 100.0).sin()) as i32;
    let data: [_; N] = with_noise(gen_function(sin), -3000..=3000);
    println!("{:?}", data);
}

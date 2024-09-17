use sort::test_data::gen_random;

fn main() {
    const N: usize = 10_000;
    let data: [_; N] = gen_random(0..N);
    println!("{:?}", data);
}

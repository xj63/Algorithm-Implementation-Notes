use sort::test_data::gen_stroll;

fn main() {
    const N: usize = 10_000;
    let data: [_; N] = gen_stroll(0, -100..=100);
    println!("{:?}", data);
}

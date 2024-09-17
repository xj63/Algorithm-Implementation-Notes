use sort::test_data::gen_stroll;

fn main() {
    const N: usize = 1000;
    let data: [_; N] = gen_stroll(0, -90..=100);
    println!("{:?}", data);
}

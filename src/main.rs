use rust_cubes::cube_generators::Cubes;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        generate(arg.parse().unwrap());
    } else {
        panic!("Expected an argument.");
    }
}


fn generate(n: usize) {
    let mut count = 0;
    for _ in Cubes::new(n) {
        count += 1;
    }
    println!("{}", count);
}

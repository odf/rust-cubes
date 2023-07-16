use rust_cubes::cube_generators::Cubes;

fn main() {
    for cubes in Cubes::new(5) {
        println!("{:?}", cubes);
    }
}

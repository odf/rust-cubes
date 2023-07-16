use rust_cubes::cube_generators::Cubes;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        call_generate(arg.parse().unwrap());
    } else {
        panic!("Expected an argument.");
    }
}




#[cfg(not(feature = "pprof"))]
fn call_generate(n: usize) {
    generate(n);
}


#[cfg(feature = "pprof")]
fn call_generate(n: usize) {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build().unwrap();

    generate(n);

    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}


fn generate(n: usize) {
    let mut count = 0;
    for _ in Cubes::new(n) {
        count += 1;
    }
    println!("{}", count);
}

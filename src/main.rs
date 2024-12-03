use std::time;
mod day01;
mod day02;
mod day03;

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}\n", time::Instant::now().duration_since(t0));

    ret
}

fn main() {
    bench(day01::run);
    bench(day02::run);
    bench(day03::run);
}

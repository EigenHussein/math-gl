use std::time::Instant;

fn main() {
    let funcs = [
        |t:f32|->f32{t},
        f32::sin,
        f32::cos,
        f32::tan,
        f32::cosh,
        f32::exp,
        f32::ln,
        f32::log2,
        f32::abs,
        f32::sqrt,
        f32::signum,
        f32::round,
        f32::floor,
        |t: f32| -> f32 { f32::powi(t, 2) },
        |t: f32| -> f32 { f32::sin(t) },
    ];

    let mut sum = 0.0;
    for f in funcs.iter() {
        let a = Instant::now();
        for i in 0..100000000 {
            let t = i as f32 / 10000000000.0;
            sum += f(t);
        }
        let b = Instant::now();
        println!("{:?} took {} millis", f, (b - a).as_millis());
    }
}

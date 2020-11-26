use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;
use rustfft::FFT;
use std::sync::Arc;
use std::time::Instant;

use num_complex;
use num_traits;

fn measure_bluestein(n: usize) -> f32 {
    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); n];
    input[0] = Complex::from(1.0);
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); n];
    let inner_fft_len = (2 * n - 1).checked_next_power_of_two().unwrap();
    let inner_fft_fw = Arc::new(rustfft::algorithm::Radix4::new(inner_fft_len, false));
    let inner_fft_inv = Arc::new(rustfft::algorithm::Radix4::new(inner_fft_len, true));
    let fft = Arc::new(rustfft::algorithm::Bluesteins::new(
        n,
        inner_fft_fw,
        inner_fft_inv,
        false,
    )) as Arc<FFT<f64>>;
    let reps = 1000000 / n + 1;
    for _r in 0..reps / 4 {
        fft.process(&mut input, &mut output);
    }
    let start = Instant::now();
    for _r in 0..reps {
        fft.process(&mut input, &mut output);
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}

fn measure_rader(n: usize) -> f32 {
    let mut input: Vec<Complex<f64>> = vec![Complex::zero(); n];
    input[0] = Complex::from(1.0);
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); n];
    let mut planner = FFTplanner::new(false);
    let inner_fft_len = n - 1;
    let inner_fft = planner.plan_fft(inner_fft_len);
    let fft = Arc::new(rustfft::algorithm::RadersAlgorithm::new(n, inner_fft)) as Arc<FFT<f64>>;

    let reps = 1000000 / n + 1;
    for _r in 0..reps / 4 {
        fft.process(&mut input, &mut output);
    }
    let start = Instant::now();
    for _r in 0..reps {
        fft.process(&mut input, &mut output);
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}

fn main() {
    println!("N, Raders, Bluesteins");
    let vals = vec![
        3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];
    for n in vals.iter() {
        let t_rader = measure_rader(*n);
        let t_bluestein = measure_bluestein(*n);
        println!("{}, {}, {}", n, t_rader, t_bluestein);
    }
}

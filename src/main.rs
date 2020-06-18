use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use std::time::Instant;
use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use chfft::CFft1D;
use num_complex;
use num_traits;

fn measure_rustfft(n:usize) -> f32 {
    let mut input:  Vec<Complex<f64>> = vec![Complex::zero(); n];
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); n];
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(n);
    let reps = 10000/n + 1;
    for _r in 0..reps/4 {
        fft.process(&mut input, &mut output);
    }
    let start = Instant::now();
    for _r in 0..reps {
        fft.process(&mut input, &mut output);
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}


fn measure_fftw(n:usize) -> f32 {
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::Measure).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let reps = 10000/n + 1;
    for _r in 0..reps/4 {
        plan.c2c(&mut a, &mut b).unwrap();
    }
    let start = Instant::now();
    for _r in 0..reps {
        plan.c2c(&mut a, &mut b).unwrap();
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}

fn measure_chfft(n:usize) -> f32 {
    let input:  Vec<num_complex::Complex<f64>> = vec![num_complex::Complex::zero(); n];
    let mut fft = CFft1D::<f64>::with_len(input.len());
    let reps = 10000/n + 1;
    for _r in 0..reps/4 {
        let output = fft.forward(&input[..]);
    }
    let start = Instant::now();
    for _r in 0..reps {
        let output = fft.forward(&input[..]);
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}

fn main() {
    println!("N, RustFFT, FFTW");
    for n in 2..256 {
        let t_rustfft = measure_rustfft(n);
        let t_fftw = measure_fftw(n);
        let t_chfft = measure_chfft(n);
        println!("{}, {}, {}, {}", n, t_rustfft, t_fftw, t_chfft);
    }
}




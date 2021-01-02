use std::{sync::Arc, time::Instant};

use num_complex::Complex;
use num_traits::Zero;
use rustfft::{Fft, FftPlannerAvx, avx::*};

fn measure_fft(fft: Arc<dyn Fft<f32>>) -> f32 {
    let mut buffer = vec![Complex::zero(); fft.len()];
    let mut scratch = vec![Complex::zero(); fft.get_inplace_scratch_len()];
    let reps = 1_000_000_000/fft.len() + 1;
    for _r in 0..(reps/20 + 1) {
        fft.process_inplace_with_scratch(&mut buffer, &mut scratch);
    }
    let start = Instant::now();
    for _r in 0..reps {
        fft.process_inplace_with_scratch(&mut buffer, &mut scratch);
    }
    let duration = start.elapsed();
    duration.as_micros() as f32 / reps as f32
}
fn main() {
    let mut planner = FftPlannerAvx::new().unwrap();
    println!("N, 2xn, 3xn, 4xn, 6xn, 8xn, 9xn, 12xn");

    let mut ns = Vec::new();
    for power2 in 2..16 {
        for power3 in 0..10 {
            for power5 in 0..10 {
                for power7 in 0..10 {
                    let n = (3usize.pow(power3) * 5usize.pow(power5) * 7usize.pow(power7)) << power2;
                    if n > 250 && n < 500000 {
                        ns.push(n);
                    }
                }
            }
        }
    }

    ns.sort();

    for n in ns {
        let base_fft = planner.plan_fft_forward(n);
        let time_base = measure_fft(Arc::clone(&base_fft));
        let time_2xn = measure_fft(Arc::new(MixedRadix2xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_3xn = measure_fft(Arc::new(MixedRadix3xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_4xn = measure_fft(Arc::new(MixedRadix4xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        //let time_5xn = measure_fft(Arc::new(MixedRadix5xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_6xn = measure_fft(Arc::new(MixedRadix6xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        //let time_7xn = measure_fft(Arc::new(MixedRadix7xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_8xn = measure_fft(Arc::new(MixedRadix8xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_9xn = measure_fft(Arc::new(MixedRadix9xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        //let time_11xn = measure_fft(Arc::new(MixedRadix11xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        let time_12xn = measure_fft(Arc::new(MixedRadix12xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        //let time_16xn = measure_fft(Arc::new(MixedRadix16xnAvx::<f32, f32>::new(Arc::clone(&base_fft)).unwrap())) / time_base;
        println!("{}, {}, {}, {}, {}, {}, {}, {}", n, time_2xn, time_3xn, time_4xn, time_6xn, time_8xn, time_9xn, time_12xn);
    }
}




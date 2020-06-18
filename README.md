Simple test program to see how the speeds of RustFFT and FFTW depend on the FFT length.

Build:
```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

Run
```
./target/release/fftbenching > results.txt
```

Plot
```
python plotresult.py results.txt
```

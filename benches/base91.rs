extern crate criterion;
extern crate base91;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use base91::{slice_encode, slice_decode};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut buf: [u8; 10240] = [0; 10240];
    let mut bytes = (0..=255).cycle();

    for i in 0..10240 {
        buf[i] = bytes.next().unwrap();
    }

    let encoded = slice_encode(&buf);

    c.bench_function("slice_encode", move |b| {
        b.iter(|| {
            slice_encode(black_box(&buf));
        })
    });

    c.bench_function("slice_decode", move |b| {
        b.iter(|| {
            slice_decode(black_box(&encoded));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hilbert_curve_rust::{CoordinateValue, HilbertCurveAlgorithm};

fn criterion_individual_benchmark(c: &mut Criterion) {
    let bits: usize = 8;
    let n: usize = 2usize.pow(bits as u32);

    c.bench_function("hilbert_curve", |b| {
        b.iter(|| {
            for x in 0..n {
                for y in 0..n {
                    hilbert_curve::convert_2d_to_1d(black_box(x), black_box(y), black_box(n));
                }
            }
        })
    });

    c.bench_function("hilbert_2d", |b| {
        b.iter(|| {
            for x in 0..n {
                for y in 0..n {
                    hilbert_2d::xy2h_discrete(
                        black_box(x),
                        black_box(y),
                        black_box(bits),
                        black_box(hilbert_2d::Variant::Hilbert),
                    );
                }
            }
        })
    });

    c.bench_function("hilbert", |b| {
        b.iter(|| {
            for x in 0..n {
                for y in 0..n {
                    let p = hilbert::Point::new(0, &[black_box(x as u32), black_box(y as u32)]);
                    p.hilbert_transform(black_box(bits));
                }
            }
        })
    });

    c.bench_function("fast_hilbert", |b| {
        b.iter(|| {
            for x in 0..n {
                for y in 0..n {
                    fast_hilbert::xy2h(black_box(x as u32), black_box(y as u32));
                }
            }
        })
    });

    c.bench_function("hilbert-curve-rust", |b| {
        let hilbert_curve = HilbertCurveAlgorithm::new(bits as u16);
        b.iter(|| {
            for x in 0..n {
                for y in 0..n {
                    hilbert_curve.point_to_index(CoordinateValue {
                        x: black_box(x as u32),
                        y: black_box(y as u32),
                    });
                }
            }
        })
    });
}

fn criterion_compare_order_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Order Benchmarks");
    for order in 6..18 {
        let row = u32::pow(2, order);
        group.bench_with_input(
            BenchmarkId::new("Hilber-Curve-Rust", order),
            &order,
            |b, order| {
                b.iter(|| {
                    let hilbert_curve = HilbertCurveAlgorithm::new(black_box(*order as u16));
                    for x in 0..row {
                        for y in 0..row {
                            hilbert_curve.point_to_index(CoordinateValue {
                                x: black_box(x as u32),
                                y: black_box(y as u32),
                            });
                        }
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("fast_hilbert", order),
            &order,
            |b, order| {
                b.iter(|| {
                    for x in 0..row {
                        for y in 0..row {
                            fast_hilbert::xy2h(black_box(x as u32), black_box(y as u32));
                        }
                    }
                })
            },
        );
/*         group.bench_with_input(BenchmarkId::new("hilbert", order), &order, |b, order| {
            b.iter(|| {
                for x in 0..row {
                    for y in 0..row {
                        let p = hilbert::Point::new(0, &[black_box(x as u32), black_box(y as u32)]);
                        p.hilbert_transform(black_box(*order as usize));
                    }
                }
            });
        }); */
        group.bench_with_input(BenchmarkId::new("hilbert_2d", order), &order, |b, order| {
            b.iter(|| {
                for x in 0..row {
                    for y in 0..row {
                        hilbert_2d::xy2h_discrete(
                            black_box(x.try_into().unwrap()),
                            black_box(y.try_into().unwrap()),
                            black_box(*order as usize),
                            black_box(hilbert_2d::Variant::Hilbert),
                        );
                    }
                }
            });
        });
        group.bench_with_input(
            BenchmarkId::new("hilbert curve", order),
            &order,
            |b, order| {
                b.iter(|| {
                    for x in 0..row {
                        for y in 0..row {
                            hilbert_curve::convert_2d_to_1d(
                                black_box(x as usize),
                                black_box(y as usize),
                                black_box(u32::pow(2, *order) as usize),
                            );
                        }
                    }
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_compare_order_benchmark, criterion_individual_benchmark
);
criterion_main!(benches);

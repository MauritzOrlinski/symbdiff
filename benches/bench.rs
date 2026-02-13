// Benchmark on execution time of growing epxressions and their derivative

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use symbdiff::expr;

fn exp_nth_derivative(c: &mut Criterion) {
    let mut group = c.benchmark_group("exp_nth_derivative");

    for n in [1, 2, 4, 8] {
        let exp = expr!((exp((add(x, 1.0)))));
        let x = stringify!(x);
        group.bench_with_input(format!("n={}", n), &n, |b, _| {
            b.iter(|| {
                black_box({
                    let mut current = exp.clone();
                    for _ in 0..n {
                        current = current.derive(x)
                    }
                    current
                })
            });
        });
    }

    group.finish();
}

criterion_group!(benches, exp_nth_derivative,);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use validate::validator::validate_move;

fn bench_validation(c: &mut Criterion) {
    c.bench_function("validate e2e4", |b| {
        b.iter(|| validate_move("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "e2e4"))
    });
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
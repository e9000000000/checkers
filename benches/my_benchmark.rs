use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/board.rs"]
mod board;

#[path = "../src/player_minmax.rs"]
mod player_minmax;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("minmax 5", |b| b.iter(|| {
        let mut bd = board::Board::new();
        player_minmax::best_move(black_box(&mut bd), 5);
    }));
    c.bench_function("minmax 7", |b| b.iter(|| {
        let mut bd = board::Board::new();
        player_minmax::best_move(black_box(&mut bd), 7);
    }));
    c.bench_function("minmax 10", |b| b.iter(|| {
        let mut bd = board::Board::new();
        player_minmax::best_move(black_box(&mut bd), 10);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

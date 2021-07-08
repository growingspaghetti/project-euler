use criterion::{criterion_group, Criterion};

criterion_group!(
    poker_scores,
    bench_poker_how_many_hands_does_player_1_win,
);

use project_euler::m54::*;

fn bench_poker_how_many_hands_does_player_1_win(c: &mut Criterion) {
    c.bench_function("poker_how_many_hands_does_player_1_win", |b| {
        b.iter(poker_how_many_hands_does_player_1_win)
    });
}

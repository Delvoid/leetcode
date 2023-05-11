use criterion::{black_box, criterion_group, criterion_main, Criterion};
use delv_leetcode::easy::valid_anagram::Solution; // replace my_crate with the name of your crate

fn bench_is_anagram(c: &mut Criterion) {
    let s = "anagram".to_string();
    let t = "nagaram".to_string();

    c.bench_function("is_anagram", |b| {
        b.iter(|| Solution::is_anagram(black_box(s.clone()), black_box(t.clone())))
    });
    c.bench_function("is_anagram_v1", |b| {
        b.iter(|| Solution::is_anagram_v1(black_box(s.clone()), black_box(t.clone())))
    });
    c.bench_function("is_anagram_v2", |b| {
        b.iter(|| Solution::is_anagram_v2(black_box(s.clone()), black_box(t.clone())))
    });
}

criterion_group!(benches, bench_is_anagram);
criterion_main!(benches);

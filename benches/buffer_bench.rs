use criterion::{black_box, criterion_group, criterion_main, Criterion};
use len_experiment::{InputBufferU8, InputBufferUsize, InputBufferU8Hint, InputBufferUsizeHint};

fn bench_push_pop_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_pop_cycle");

    group.bench_function("u8", |b| {
        b.iter(|| {
            let mut buf = InputBufferU8::new();
            for i in 0..13u32 {
                let _ = buf.push(char::from_u32(65 + i).unwrap());
            }
            for _ in 0..13 {
                black_box(buf.pop());
            }
        })
    });

    group.bench_function("usize", |b| {
        b.iter(|| {
            let mut buf = InputBufferUsize::new();
            for i in 0..13u32 {
                let _ = buf.push(char::from_u32(65 + i).unwrap());
            }
            for _ in 0..13 {
                black_box(buf.pop());
            }
        })
    });

    group.bench_function("u8_hint", |b| {
        b.iter(|| {
            let mut buf = InputBufferU8Hint::new();
            for i in 0..13u32 {
                let _ = buf.push(char::from_u32(65 + i).unwrap());
            }
            for _ in 0..13 {
                black_box(buf.pop());
            }
        })
    });

    group.bench_function("usize_hint", |b| {
        b.iter(|| {
            let mut buf = InputBufferUsizeHint::new();
            for i in 0..13u32 {
                let _ = buf.push(char::from_u32(65 + i).unwrap());
            }
            for _ in 0..13 {
                black_box(buf.pop());
            }
        })
    });

    group.finish();
}

fn bench_random_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_access");

    // Pre-fill buffers
    let mut buf_u8 = InputBufferU8::new();
    let mut buf_usize = InputBufferUsize::new();
    let mut buf_u8_hint = InputBufferU8Hint::new();
    let mut buf_usize_hint = InputBufferUsizeHint::new();
    for i in 0..13u32 {
        let c = char::from_u32(65 + i).unwrap();
        let _ = buf_u8.push(c);
        let _ = buf_usize.push(c);
        let _ = buf_u8_hint.push(c);
        let _ = buf_usize_hint.push(c);
    }

    // Deterministic pseudo-random index sequence
    let indices: Vec<usize> = (0..1000).map(|i| (i * 7) % 13).collect();

    group.bench_function("u8", |b| {
        b.iter(|| {
            for &i in &indices {
                black_box(buf_u8.get(i));
            }
        })
    });

    group.bench_function("usize", |b| {
        b.iter(|| {
            for &i in &indices {
                black_box(buf_usize.get(i));
            }
        })
    });

    group.bench_function("u8_hint", |b| {
        b.iter(|| {
            for &i in &indices {
                black_box(buf_u8_hint.get(i));
            }
        })
    });

    group.bench_function("usize_hint", |b| {
        b.iter(|| {
            for &i in &indices {
                black_box(buf_usize_hint.get(i));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_push_pop_cycle, bench_random_access);
criterion_main!(benches);

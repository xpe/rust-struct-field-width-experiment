# Experiment: `u8` vs `usize` for Buffer Length Field

## Objective

Compare the runtime characteristics of using `u8` versus `usize` for the `len` field in a fixed-capacity `InputBuffer` struct. Focus on:

1. Assembly output differences (particularly zero-extension instructions)
2. Effect of `std::hint::assert_unchecked` on codegen
3. Microbenchmark results under tight loops

## Setup

Create a new Rust project:

```bash
cargo init --name len_experiment
```

Add to `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "buffer_bench"
harness = false
```

## Implementation

Create `src/lib.rs` with four variants:

1. `InputBufferU8` — uses `len: u8`
2. `InputBufferUsize` — uses `len: usize`
3. `InputBufferU8Hint` — uses `len: u8` with `assert_unchecked(self.len <= 13)` before indexing
4. `InputBufferUsizeHint` — uses `len: usize` with the same hint (control case)

Each variant should implement:
- `new() -> Self`
- `push(&mut self, c: char) -> Result<(), ()>`
- `pop(&mut self) -> Option<char>`
- `get(&self, index: usize) -> Option<&char>`
- `len(&self) -> usize`

## Phase 1: Assembly Comparison

**For this phase only**, mark all methods `#[inline(never)]`. This prevents inlining so that `cargo-show-asm` can display each function's assembly in isolation. Without this, these small methods will be inlined at call sites and won't exist as standalone symbols.

Use `cargo-show-asm` to inspect each variant:

```bash
cargo asm --lib len_experiment::InputBufferU8::push
cargo asm --lib len_experiment::InputBufferU8::get
cargo asm --lib len_experiment::InputBufferUsize::push
cargo asm --lib len_experiment::InputBufferUsize::get
cargo asm --lib len_experiment::InputBufferU8Hint::push
cargo asm --lib len_experiment::InputBufferU8Hint::get
cargo asm --lib len_experiment::InputBufferUsizeHint::push
cargo asm --lib len_experiment::InputBufferUsizeHint::get
```

The `--rust` flag interleaves source lines with assembly, which helps map instructions to code:

```bash
cargo asm --lib --rust len_experiment::InputBufferU8::push
```

### What to Look For

1. Count `movzx` or `movzbl` instructions (zero-extension from 8-bit to 64-bit)
2. Count total instructions per function
3. Note any bounds-check differences (look for panic/unwrap paths)
4. Check if `assert_unchecked` eliminates any bounds checks

Create a markdown table summarizing:

| Variant   | Method | Total Instructions | Zero-Extensions | Bounds Check Present |
|-----------|--------|-------------------:|----------------:|----------------------|
| U8        | push   |                  ? |               ? | ?                    |
| U8        | get    |                  ? |               ? | ?                    |
| Usize     | push   |                  ? |               ? | ?                    |
| Usize     | get    |                  ? |               ? | ?                    |
| U8Hint    | push   |                  ? |               ? | ?                    |
| U8Hint    | get    |                  ? |               ? | ?                    |
| UsizeHint | push   |                  ? |               ? | ?                    |
| UsizeHint | get    |                  ? |               ? | ?                    |

## Phase 2: Microbenchmarks

**Remove all `#[inline(never)]` attributes before benchmarking.** The Phase 1 attribute prevents inlining, which creates unrealistic conditions for performance measurement. Real-world performance depends on inlined behavior.

Create `benches/buffer_bench.rs`:

```rust
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
```

Run with:

```bash
cargo bench
```

## Phase 3: Struct Size and Alignment Verification

Add a test or print statement:

```rust
println!("InputBufferU8:    size={}, align={}", 
    std::mem::size_of::<InputBufferU8>(), 
    std::mem::align_of::<InputBufferU8>());
println!("InputBufferUsize: size={}, align={}", 
    std::mem::size_of::<InputBufferUsize>(), 
    std::mem::align_of::<InputBufferUsize>());
```

## Deliverables

After running the experiment, produce a summary file `RESULTS.md` containing:

1. The assembly comparison table from Phase 1
2. Benchmark results (copy the criterion output or summarize)
3. Size/alignment verification output
4. A brief analysis section noting:
   - Whether zero-extension instructions are present and how many
   - Whether `assert_unchecked` changed anything
   - Whether benchmark differences are measurable or within noise
   - Any surprising findings

Do not make a recommendation. Present the data.

## Notes

- Use Rust 1.81+ (for stable `assert_unchecked`)
- Run benchmarks multiple times; criterion handles statistical analysis
- Ensure the machine is relatively idle during benchmarks

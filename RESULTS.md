# Results: u8 vs usize for Buffer Length Field

**Platform:** Apple Silicon (ARM64/aarch64)
**Rust version:** See `rustc --version`
**Date:** 2025-12-23

## Phase 1: Assembly Comparison

Assembly captured with `#[inline(never)]` attributes to prevent inlining.

**Note:** This experiment ran on ARM64 (Apple Silicon), not x86-64. On ARM64, `ldrb` (load byte) implicitly zero-extends to a 32-bit register, so there are no separate `movzx`/`movzbl` instructions. The table notes where zero-extension occurs implicitly via the load instruction.

| Variant   | Method | Instructions (happy path) | Zero-Extension | Bounds Check Present |
|-----------|--------|---------------------------|----------------|----------------------|
| U8        | push   | 10                        | implicit (ldrb) | No (capacity only)  |
| U8        | get    | 6 + panic path            | implicit (ldrb) | Yes                  |
| Usize     | push   | 10                        | none            | No (capacity only)  |
| Usize     | get    | 7 + panic path            | none            | Yes                  |
| U8Hint    | push   | 10                        | implicit (ldrb) | No                   |
| U8Hint    | get    | 5                         | implicit (ldrb) | **No (eliminated)**  |
| UsizeHint | push   | 10                        | none            | No                   |
| UsizeHint | get    | 6                         | none            | **No (eliminated)**  |

### Assembly Listings

#### InputBufferU8::push
```asm
ldrb w8, [x0, #52]        ; load len (byte)
cmp x8, #13
b.hs LBB4_2               ; if len >= 13, skip
str w1, [x0, x8, lsl #2]  ; store char at data[len]
ldrb w9, [x0, #52]        ; reload len
add w9, w9, #1
strb w9, [x0, #52]        ; store len + 1
LBB4_2:
cmp w8, #12
cset w0, hi               ; return Err if was >= 13
ret
```

#### InputBufferU8::get (without hint)
```asm
ldrb w8, [x0, #52]        ; load len
cmp x1, x8                ; compare index to len
b.hs LBB0_3               ; if index >= len, return None
cmp x1, #13               ; BOUNDS CHECK
b.hs LBB0_4               ; if index >= 13, panic
add x0, x0, x1, lsl #2
ret
LBB0_3:
mov x0, #0
ret
LBB0_4:
; ... panic path ...
```

#### InputBufferU8Hint::get (with assert_unchecked)
```asm
ldrb w8, [x0, #52]        ; load len
add x9, x0, x1, lsl #2    ; compute address speculatively
cmp x1, x8                ; compare index to len
csel x0, x9, xzr, lo      ; select address or null
ret
```

#### InputBufferUsize::push
```asm
ldr x8, [x0]              ; load len (qword)
cmp x8, #13
b.hs LBB9_2
add x9, x0, x8, lsl #2
str w1, [x9, #8]          ; store char at data[len]
add x9, x8, #1
str x9, [x0]              ; store len + 1
LBB9_2:
cmp x8, #12
cset w0, hi
ret
```

#### InputBufferUsizeHint::get (with assert_unchecked)
```asm
ldr x8, [x0]              ; load len
add x9, x0, x1, lsl #2
add x9, x9, #8
cmp x1, x8
csel x0, xzr, x9, hs      ; select null or address
ret
```

## Phase 2: Benchmark Results

Benchmarks run with inlining enabled (no `#[inline(never)]`).

### push_pop_cycle (13 pushes + 13 pops)

| Variant    | Time (ns)       | Relative |
|------------|-----------------|----------|
| u8         | 12.86 - 12.89   | 3.5x     |
| usize      | 3.65 - 3.66     | 1.0x     |
| u8_hint    | 12.91 - 13.33   | 3.6x     |
| usize_hint | 3.66 - 3.66     | 1.0x     |

### random_access (1000 get() calls)

| Variant    | Time (ns)       | Relative |
|------------|-----------------|----------|
| u8         | 274.9 - 292.6   | 1.0x - 1.1x |
| usize      | 258.8 - 263.5   | 1.0x     |
| u8_hint    | 260.5 - 261.6   | 1.0x     |
| usize_hint | 260.9 - 261.8   | 1.0x     |

## Phase 3: Size and Alignment

```
InputBufferU8:        size=56, align=4
InputBufferUsize:     size=64, align=8
InputBufferU8Hint:    size=56, align=4
InputBufferUsizeHint: size=64, align=8
```

The U8 variants are 8 bytes smaller due to the `u8` len field and reduced alignment requirements.

## Analysis

### Zero-Extension Instructions

On ARM64, there are no separate zero-extension instructions. The `ldrb` instruction (load byte) implicitly zero-extends to a 32-bit register. When used in 64-bit comparisons (e.g., `cmp x8, #13`), the upper 32 bits are already zero.

On x86-64, you would expect to see `movzx` or `movzbl` instructions for the U8 variants.

### Effect of assert_unchecked

`assert_unchecked` eliminated array bounds checks in the `get()` method:

- **Without hint:** The compiler generates a bounds check (`cmp x1, #13; b.hs panic`) because it cannot prove `index < 13` even after checking `index < len`.
- **With hint:** The assertion `len <= 13` allows the compiler to elide the bounds check, since `index < len` and `len <= 13` implies `index < 13`.

The `push()` method showed no change, as the capacity check already provides sufficient information.

### Benchmark Observations

1. **push_pop_cycle:** The usize variants are approximately 3.5x faster than the u8 variants. This is a substantial difference that persists regardless of the hint.

2. **random_access:** Without the hint, u8 is slightly slower (~10-15 ns overhead over 1000 calls). With hints, all variants converge to similar performance (~261 ns).

3. The push_pop_cycle result suggests that repeated u8 operations (load-modify-store of a byte) have significant overhead compared to native word operations, possibly due to partial register stalls or the need for additional instructions.

### Surprising Findings

- The magnitude of the push_pop_cycle difference (3.5x) was unexpected. This suggests the cost of working with sub-word types in tight loops can be significant on ARM64.
- The `assert_unchecked` hint did not improve push_pop_cycle performance for u8, indicating the bottleneck is not bounds checking but rather the u8 operations themselves.
- For random_access, the hint brought u8 to parity with usize, eliminating the penalty entirely.

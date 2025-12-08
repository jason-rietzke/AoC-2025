# 🎄 Advent of Code 2025 🎄

Welcome to my Advent of Code 2025 solutions! This repository contains my attempts at solving the daily programming puzzles from [Advent of Code 2025](https://adventofcode.com/2025).

## 📊 System Specifications

- **CPU:** AMD Ryzen AI 9 HX 370 w/ Radeon 890M
- **Cores:** 24
- **OS:** Arch Linux

## 🏃 Performance Results

All solutions are implemented in Rust and compiled with `--release` optimizations.

### Day 1

```
Part 1: 1168 (in 620.94µs)
Part 2: 7199 (in 1.072485ms)
```

*Total time: 5.387 ms*

### Day 2

```
Part 1: 30599400849 (in 35.973231ms)
Part 2: 46270373595 (in 66.822115ms)
```

*Total time: 107.015 ms*

### Day 3

```
Part 1: 17158 (in 401.2µs)
Part 2: 170449335646486 (in 555.849µs)
```

*Total time: 4.594 ms*

### Day 4

```
Part 1: 1489 (in 496.488µs)
Part 2: 8890 (in 2.172681ms)
```

*Total time: 5.468 ms*

### Day 5

```
Part 1: 775 (in 195.506µs)
Part 2: 350684792662845 (in 60.603µs)
```

*Total time: 3.090 ms*

### Day 6

```
Part 1: 4722948564882 (in 217.717µs)
Part 2: 9581313737063 (in 407.321µs)
```

*Total time: 3.711 ms*

### Day 7

```
Part 1: 1690 (in 234.699µs)
Part 2: 221371496188107 (in 247.102µs)
```

*Total time: 3.376 ms*

### Day 8

```
Part 1: 163548 (in 35.493475ms)
Part 2: 772452514 (in 263.26912ms)
```

*Total time: 304.350 ms*


---

**Total Runtime:** 500.454 ms

## 🛠️ Building and Running

To build all solutions:
```bash
cargo build --release
```

To run all days:
```bash
./run-all.sh
```

To run a specific day:
```bash
cargo run -p day01 --release
```

## 📝 Notes

Each day's solution is organized in its own workspace member with:
- `src/main.rs` - The solution code
- `data.txt` - The puzzle input
- `Cargo.toml` - Day-specific dependencies

---

*Last updated: 2025-12-08 19:03:07*

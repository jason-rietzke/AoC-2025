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
Part 1: 1168 (in 608.342µs)
Part 2: 7199 (in 1.067383ms)
```

*Total time: 3.757 ms*

### Day 2

```
Part 1: 30599400849 (in 33.23266ms)
Part 2: 46270373595 (in 66.293378ms)
```

*Total time: 103.469 ms*

### Day 3

```
Part 1: 17158 (in 391.905µs)
Part 2: 170449335646486 (in 545.885µs)
```

*Total time: 3.489 ms*

### Day 4

```
Part 1: 1489 (in 199.364µs)
Part 2: 8890 (in 1.355333ms)
```

*Total time: 3.642 ms*

### Day 5

```
Part 1: 775 (in 194.344µs)
Part 2: 350684792662845 (in 58.66µs)
```

*Total time: 2.019 ms*

### Day 6

```
Part 1: 4722948564882 (in 208.571µs)
Part 2: 9581313737063 (in 414.788µs)
```

*Total time: 2.608 ms*

### Day 7

```
Part 1: 1690 (in 230.122µs)
Part 2: 221371496188107 (in 168.597µs)
```

*Total time: 2.152 ms*


---

**Total Runtime:** 167.869 ms

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

*Last updated: 2025-12-07 08:06:58*

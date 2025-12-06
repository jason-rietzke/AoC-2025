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
Part 1: 1168 (in 610.423µs)
Part 2: 7199 (in 1.095119ms)
```

*Total time: 3.709 ms*

### Day 2

```
Part 1: 30599400849 (in 33.579156ms)
Part 2: 46270373595 (in 66.308095ms)
```

*Total time: 103.034 ms*

### Day 3

```
Part 1: 17158 (in 413.371µs)
Part 2: 170449335646486 (in 544.669µs)
```

*Total time: 3.230 ms*

### Day 4

```
Part 1: 1489 (in 298.474µs)
Part 2: 8890 (in 2.057267ms)
```

*Total time: 4.279 ms*

### Day 5

```
Part 1: 775 (in 189.298µs)
Part 2: 350684792662845 (in 58.22µs)
```

*Total time: 2.075 ms*

### Day 6

```
Part 1: 4722948564882 (in 208.243µs)
Part 2: 9581313737063 (in 534.309µs)
```

*Total time: 2.537 ms*


---

**Total Runtime:** 159.110 ms

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

*Last updated: 2025-12-06 13:32:34*

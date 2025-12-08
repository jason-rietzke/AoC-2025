# 🎄 Advent of Code 2025 🎄

Welcome to my Advent of Code 2025 solutions! This repository contains my attempts at solving the daily programming puzzles from [Advent of Code 2025](https://adventofcode.com/2025).

## 📊 System Specifications

- **CPU:** AMD Ryzen 9 7950X3D 16-Core Processor
- **Cores:** 32
- **OS:** Arch Linux

## 🏃 Performance Results

All solutions are implemented in Rust and compiled with `--release` optimizations.

### Day 1

```
Part 1: 1168 (in 329.427µs)
Part 2: 7199 (in 527.083µs)
```

*Total time: 2.618 ms*

### Day 2

```
Part 1: 30599400849 (in 34.413453ms)
Part 2: 46270373595 (in 72.323074ms)
```

*Total time: 108.847 ms*

### Day 3

```
Part 1: 17158 (in 226.141µs)
Part 2: 170449335646486 (in 321.843µs)
```

*Total time: 1.961 ms*

### Day 4

```
Part 1: 1489 (in 156.308µs)
Part 2: 8890 (in 928.057µs)
```

*Total time: 2.525 ms*

### Day 5

```
Part 1: 775 (in 112.504µs)
Part 2: 350684792662845 (in 34.064µs)
```

*Total time: 1.603 ms*

### Day 6

```
Part 1: 4722948564882 (in 119.066µs)
Part 2: 9581313737063 (in 288.268µs)
```

*Total time: 1.906 ms*

### Day 7

```
Part 1: 1690 (in 171.877µs)
Part 2: 221371496188107 (in 127.673µs)
```

*Total time: 1.703 ms*

### Day 8

```
Part 1: 163548 (in 10.710734ms)
	Parsing: 58.822µs
	Calculating links: 9.358178ms
	Calculating circuits: 1.282411ms
	Evaluating result: 4.599µs
Part 2: 772452514 (in 17.387035ms)
	Parsing: 57.64µs
	Calculating links: 15.169653ms
	Calculating circuits: 2.068739ms
	Evaluating result: 20ns
```

*Total time: 29.784 ms*


---

**Total Runtime:** 196.974 ms

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

*Last updated: 2025-12-08 21:23:22*

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
Part 1: 1168 (in 330.268µs)
Part 2: 7199 (in 523.775µs)
```

*Total time: 2.593 ms*

### Day 2

```
Part 1: 30599400849 (in 34.620148ms)
Part 2: 46270373595 (in 72.747378ms)
```

*Total time: 109.335 ms*

### Day 3

```
Part 1: 17158 (in 229.316µs)
Part 2: 170449335646486 (in 338.544µs)
```

*Total time: 2.419 ms*

### Day 4

```
Part 1: 1489 (in 163.861µs)
Part 2: 8890 (in 947.73µs)
```

*Total time: 2.970 ms*

### Day 5

```
Part 1: 775 (in 107.494µs)
Part 2: 350684792662845 (in 34.896µs)
```

*Total time: 2.030 ms*

### Day 6

```
Part 1: 4722948564882 (in 115.129µs)
Part 2: 9581313737063 (in 308.306µs)
```

*Total time: 2.100 ms*

### Day 7

```
Part 1: 1690 (in 176.755µs)
Part 2: 221371496188107 (in 120.629µs)
```

*Total time: 1.995 ms*

### Day 8

```
Part 1: 163548 (in 10.546668ms)
	Parsing: 58.741µs
	Calculating links: 9.218605ms
	Calculating circuits: 1.259764ms
	Evaluating result: 4.208µs
Part 2: 772452514 (in 17.046899ms)
	Parsing: 58.572µs
	Calculating links: 14.822853ms
	Calculating circuits: 2.074271ms
	Evaluating result: 20ns
```

*Total time: 29.461 ms*

### Day 9

```
Part 1: 4782896435 (in 274.912µs)
Part 2: 1540060480 (in 21.938933ms)
```

*Total time: 24.055 ms*

### Day 10

```
Part 1: 469 (in 3.532742ms)
Part 2: 19293 (in 127.758247ms)
```

*Total time: 137.103 ms*

### Day 11

```
Part 1: 552 (in 108.286µs)
Part 2: 307608674109300 (in 152.239µs)
```

*Total time: 2.043 ms*

### Day 12

```
Part 1: 591 (in 147.21µs)
Part 2: Merry Christmas! (in 20ns)
```

*Total time: 1.849 ms*


---

**Total Runtime:** 388.499 ms

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

*Last updated: 2025-12-12 22:53:55*

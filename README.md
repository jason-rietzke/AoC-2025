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
Part 1: 1168 (in 446.699µs)
Part 2: 7199 (in 526.381µs)
```

*Total time: 2.695 ms*

### Day 2

```
Part 1: 30599400849 (in 34.395777ms)
Part 2: 46270373595 (in 72.363696ms)
```

*Total time: 108.585 ms*

### Day 3

```
Part 1: 17158 (in 227.372µs)
Part 2: 170449335646486 (in 327.473µs)
```

*Total time: 2.354 ms*

### Day 4

```
Part 1: 1489 (in 154.244µs)
Part 2: 8890 (in 1.001534ms)
```

*Total time: 2.981 ms*

### Day 5

```
Part 1: 775 (in 103.657µs)
Part 2: 350684792662845 (in 33.343µs)
```

*Total time: 1.991 ms*

### Day 6

```
Part 1: 4722948564882 (in 113.145µs)
Part 2: 9581313737063 (in 356.478µs)
```

*Total time: 2.092 ms*

### Day 7

```
Part 1: 1690 (in 178.419µs)
Part 2: 221371496188107 (in 123.896µs)
```

*Total time: 1.702 ms*

### Day 8

```
Part 1: 163548 (in 10.647561ms)
	Parsing: 67.178µs
	Calculating links: 9.332281ms
	Calculating circuits: 1.237612ms
	Evaluating result: 4.709µs
Part 2: 772452514 (in 18.376755ms)
	Parsing: 77.568µs
	Calculating links: 16.166653ms
	Calculating circuits: 2.046751ms
	Evaluating result: 91ns
```

*Total time: 30.682 ms*

### Day 9

```
Part 1: 4782896435 (in 273.34µs)
Part 2: 1540060480 (in 22.029259ms)
```

*Total time: 24.177 ms*

### Day 10

```
Part 1: 469 (in 3.436452ms)
Part 2: 19293 (in 127.250668ms)
```

*Total time: 135.970 ms*


---

**Total Runtime:** 372.183 ms

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

*Last updated: 2025-12-11 00:39:46*

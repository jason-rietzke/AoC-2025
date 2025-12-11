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
Part 1: 1168 (in 330.859µs)
Part 2: 7199 (in 525.629µs)
```

*Total time: 2.721 ms*

### Day 2

```
Part 1: 30599400849 (in 34.677084ms)
Part 2: 46270373595 (in 69.286529ms)
```

*Total time: 105.936 ms*

### Day 3

```
Part 1: 17158 (in 224.617µs)
Part 2: 170449335646486 (in 323.756µs)
```

*Total time: 2.277 ms*

### Day 4

```
Part 1: 1489 (in 148.493µs)
Part 2: 8890 (in 887.938µs)
```

*Total time: 2.770 ms*

### Day 5

```
Part 1: 775 (in 108.025µs)
Part 2: 350684792662845 (in 33.664µs)
```

*Total time: 1.860 ms*

### Day 6

```
Part 1: 4722948564882 (in 164.002µs)
Part 2: 9581313737063 (in 289.56µs)
```

*Total time: 2.305 ms*

### Day 7

```
Part 1: 1690 (in 176.355µs)
Part 2: 221371496188107 (in 130.518µs)
```

*Total time: 2.014 ms*

### Day 8

```
Part 1: 163548 (in 10.603365ms)
	Parsing: 59.764µs
	Calculating links: 9.288335ms
	Calculating circuits: 1.224257ms
	Evaluating result: 5.19µs
Part 2: 772452514 (in 17.033777ms)
	Parsing: 53.832µs
	Calculating links: 14.88318ms
	Calculating circuits: 2.010321ms
	Evaluating result: 20ns
```

*Total time: 29.545 ms*

### Day 9

```
Part 1: 4782896435 (in 280.573µs)
Part 2: 1540060480 (in 21.43842ms)
```

*Total time: 23.498 ms*

### Day 10

```
Part 1: 469 (in 3.356911ms)
Part 2: 19293 (in 127.168179ms)
```

*Total time: 136.697 ms*

### Day 11

```
Part 1: 552 (in 109.738µs)
Part 2: 307608674109300 (in 171.295µs)
```

*Total time: 2.276 ms*


---

**Total Runtime:** 375.824 ms

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

*Last updated: 2025-12-12 00:23:32*

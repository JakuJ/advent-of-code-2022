<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

All solutions marked with ⭐ run in **under 1ms** in release mode (on my machine, your experience may vary).
Solutions marked ✨ take longer than that.

|                      Day                       | Part 1  |  Part 2  |
| :--------------------------------------------: | :-----: | :------: |
|  [Day 1](https://adventofcode.com/2022/day/1)  |    ⭐    |    ⭐     |
|  [Day 2](https://adventofcode.com/2022/day/2)  |    ⭐    |    ⭐     |
|  [Day 3](https://adventofcode.com/2022/day/3)  |    ⭐    |    ⭐     |
|  [Day 4](https://adventofcode.com/2022/day/4)  |    ⭐    |    ⭐     |
|  [Day 5](https://adventofcode.com/2022/day/5)  |    ⭐    |    ⭐     |
|  [Day 6](https://adventofcode.com/2022/day/6)  |    ⭐    |    ⭐     |
|  [Day 7](https://adventofcode.com/2022/day/7)  |    ⭐    |    ⭐     |
|  [Day 8](https://adventofcode.com/2022/day/8)  |    ⭐    |    ⭐     |
|  [Day 9](https://adventofcode.com/2022/day/9)  |    ⭐    |    ⭐     |
| [Day 10](https://adventofcode.com/2022/day/10) |    ⭐    |    ⭐     |
| [Day 11](https://adventofcode.com/2022/day/11) |    ⭐    | ✨ 1.1 ms |
| [Day 12](https://adventofcode.com/2022/day/12) | ✨ 24 ms | ✨ 22 ms  |
| [Day 13](https://adventofcode.com/2022/day/13) |    ⭐    |    ⭐     |
| [Day 14](https://adventofcode.com/2022/day/14) |    ⭐    | ✨ 2.9 ms |
| [Day 15](https://adventofcode.com/2022/day/15) |    ⭐    | ✨ 20 ms  |
| [Day 16](https://adventofcode.com/2022/day/16) |         |          |
| [Day 17](https://adventofcode.com/2022/day/17) |         |          |
| [Day 18](https://adventofcode.com/2022/day/18) |    ⭐    | ✨ 1.9 ms |
| [Day 19](https://adventofcode.com/2022/day/19) |         |          |
| [Day 20](https://adventofcode.com/2022/day/20) | ✨ 40 ms | ✨ 645 ms |
| [Day 21](https://adventofcode.com/2022/day/21) |    ⭐    | ✨ 25 ms  |

---

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. I yse these unit tests to develop and debug my solution against the example input.

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloading input with aoc-cli...
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2022...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of the solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against the example input

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

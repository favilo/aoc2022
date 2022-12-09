# Advent of Code 2022

To run all days:

```sh
cargo run --release
```

Timings generated by:

The `cargo-criterion` crate is useful to get nice benchmarks.

```sh
cargo criterion
```

Though not required, this just doesn't have as nice output, and will deprecate plots soon:

```sh
cargo bench
```

## Timings

```
day01/get_input         time:   [43.604 µs 43.731 µs 43.906 µs]
day01/part1             time:   [1.1634 ns 1.1662 ns 1.1693 ns]
day01/part2             time:   [2.6796 ns 2.6848 ns 2.6903 ns]

day02/get_input         time:   [51.790 µs 51.953 µs 52.126 µs]
day02/part1             time:   [7.5178 µs 7.5647 µs 7.6226 µs]
day02/part2             time:   [14.886 µs 14.970 µs 15.085 µs]

day03/get_input         time:   [27.765 µs 27.930 µs 28.129 µs]
day03/part1             time:   [457.56 ns 459.16 ns 461.40 ns]
day03/part2             time:   [216.41 ns 222.24 ns 232.58 ns]

day04/get_input         time:   [61.826 µs 63.037 µs 64.460 µs]
day04/part1             time:   [1.8978 µs 1.9409 µs 2.0004 µs]
day04/part2             time:   [2.0602 µs 2.1320 µs 2.1998 µs]

day05/get_input         time:   [19.999 µs 20.221 µs 20.468 µs]
day05/part1             time:   [6.0702 µs 6.1162 µs 6.1735 µs]
day05/part2             time:   [6.6191 µs 6.7431 µs 6.8672 µs]

day06/get_input         time:   [1.1940 ns 1.2045 ns 1.2151 ns]
day06/part1             time:   [4.1462 µs 4.1780 µs 4.2106 µs]
day06/part2             time:   [6.1832 µs 6.2897 µs 6.4200 µs]

day07/get_input         time:   [123.41 µs 123.69 µs 124.04 µs]
day07/part1             time:   [17.759 µs 17.776 µs 17.796 µs]
day07/part2             time:   [19.417 µs 19.475 µs 19.541 µs]

day08/get_input         time:   [196.15 µs 198.41 µs 200.85 µs]
day08/part1             time:   [41.989 µs 42.246 µs 42.560 µs]
day08/part2             time:   [419.24 µs 423.74 µs 429.84 µs]

```

## Original Timings

```
day01/get_input         time:   [86.597 µs 86.773 µs 86.980 µs]
day01/part1             time:   [811.66 ns 813.55 ns 815.58 ns]
day01/part2             time:   [5.3042 µs 5.3207 µs 5.3401 µs]

day02/get_input         time:   [155.28 µs 157.07 µs 159.38 µs]
day02/part1             time:   [10.627 µs 10.668 µs 10.719 µs]
day02/part2             time:   [9.6086 µs 9.6317 µs 9.6583 µs]

day03/get_input         time:   [91.904 µs 93.008 µs 94.309 µs]
day03/part1             time:   [230.05 µs 234.30 µs 239.73 µs]
day03/part2             time:   [253.34 µs 256.95 µs 261.45 µs]

day04/get_input         time:   [53.297 µs 53.618 µs 54.137 µs]
day04/part1             time:   [1.2248 µs 1.2271 µs 1.2296 µs]
day04/part2             time:   [2.1796 µs 2.1856 µs 2.1924 µs]

day05/get_input         time:   [33.789 µs 33.987 µs 34.206 µs]
day05/part1             time:   [5.4411 µs 5.5016 µs 5.5739 µs]
day05/part2             time:   [14.364 µs 14.828 µs 15.247 µs]

day06/get_input         time:   [69.029 ns 69.081 ns 69.135 ns]
day06/part1             time:   [44.392 µs 44.491 µs 44.585 µs]
day06/part2             time:   [201.74 µs 203.76 µs 206.72 µs]

day07/get_input         time:   [180.59 µs 180.68 µs 180.77 µs]
day07/part1             time:   [157.61 µs 157.81 µs 158.06 µs]
day07/part2             time:   [180.45 µs 181.55 µs 183.00 µs]

day08/get_input         time:   [202.26 µs 205.49 µs 209.69 µs]
day08/part1             time:   [863.33 µs 865.17 µs 867.44 µs]
day08/part2             time:   [1.0570 ms 1.0590 ms 1.0614 ms]

```

## Failed experiments

### Day 5

Attempting to move all the stuff from Vec's to the stack with heapless ended up giving me the following worse timings:

```
day05/get_input         time:   [31.051 µs 31.192 µs 31.307 µs]
                        change: [-3.5626% -1.3168% +0.4653%] (p = 0.23 > 0.05)
                        No change in performance detected.
day05/part1             time:   [10.293 µs 10.303 µs 10.315 µs]
                        change: [+70.356% +73.084% +75.661%] (p = 0.00 < 0.05)
                        Performance has regressed.
day05/part2             time:   [10.931 µs 10.970 µs 11.023 µs]
                        change: [+92.267% +93.888% +95.383%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

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
day01/get_input         time:   [45.440 µs 45.478 µs 45.522 µs]
                        change: [+1.9923% +2.9710% +3.8084%] (p = 0.00 < 0.05)
                        Performance has regressed.
day01/part1             time:   [1.1156 ns 1.1197 ns 1.1259 ns]
                        change: [-5.0302% -4.5179% -4.0219%] (p = 0.00 < 0.05)
                        Performance has improved.
day01/part2             time:   [2.6443 ns 2.6662 ns 2.7068 ns]
                        change: [-1.4148% -0.5309% +0.4163%] (p = 0.27 > 0.05)
                        No change in performance detected.

day02/get_input         time:   [51.659 µs 51.809 µs 51.964 µs]
                        change: [-0.1457% +0.4203% +0.9468%] (p = 0.14 > 0.05)
                        No change in performance detected.
day02/part1             time:   [7.8771 µs 7.8896 µs 7.9057 µs]
                        change: [+1.9766% +3.4134% +4.5541%] (p = 0.00 < 0.05)
                        Performance has regressed.
day02/part2             time:   [13.932 µs 13.939 µs 13.947 µs]
                        change: [-8.5967% -7.4716% -6.6586%] (p = 0.00 < 0.05)
                        Performance has improved.

day03/get_input         time:   [25.679 µs 25.760 µs 25.849 µs]
                        change: [-8.1937% -7.5067% -6.9269%] (p = 0.00 < 0.05)
                        Performance has improved.
day03/part1             time:   [498.44 ns 499.34 ns 500.57 ns]
                        change: [+8.0327% +8.9041% +9.6342%] (p = 0.00 < 0.05)
                        Performance has regressed.
day03/part2             time:   [211.04 ns 211.24 ns 211.50 ns]
                        change: [-5.4108% -3.5647% -2.3482%] (p = 0.00 < 0.05)
                        Performance has improved.

day04/get_input         time:   [57.075 µs 57.106 µs 57.140 µs]
                        change: [-9.7980% -8.4254% -7.1591%] (p = 0.00 < 0.05)
                        Performance has improved.
day04/part1             time:   [1.5199 µs 1.5232 µs 1.5274 µs]
                        change: [-21.541% -20.277% -19.240%] (p = 0.00 < 0.05)
                        Performance has improved.
day04/part2             time:   [1.6933 µs 1.6984 µs 1.7041 µs]
                        change: [-17.061% -15.462% -13.940%] (p = 0.00 < 0.05)
                        Performance has improved.

day05/get_input         time:   [18.234 µs 18.269 µs 18.311 µs]
                        change: [-11.246% -9.9724% -8.8016%] (p = 0.00 < 0.05)
                        Performance has improved.
day05/part1             time:   [5.0598 µs 5.0722 µs 5.0885 µs]
                        change: [-18.539% -17.602% -16.782%] (p = 0.00 < 0.05)
                        Performance has improved.
day05/part2             time:   [5.2125 µs 5.2180 µs 5.2247 µs]
                        change: [-20.884% -19.533% -18.229%] (p = 0.00 < 0.05)
                        Performance has improved.

day06/get_input         time:   [1.1336 ns 1.1357 ns 1.1386 ns]
                        change: [-6.8249% -5.6127% -4.5323%] (p = 0.00 < 0.05)
                        Performance has improved.
day06/part1             time:   [3.9257 µs 3.9277 µs 3.9302 µs]
                        change: [-7.1613% -6.1279% -5.1845%] (p = 0.00 < 0.05)
                        Performance has improved.
day06/part2             time:   [5.9778 µs 6.0007 µs 6.0380 µs]
                        change: [-18.250% -12.527% -7.0356%] (p = 0.00 < 0.05)
                        Performance has improved.

day07/get_input         time:   [126.19 µs 126.56 µs 127.04 µs]
                        change: [-6.2881% -5.6931% -4.9627%] (p = 0.00 < 0.05)
                        Performance has improved.
day07/part1             time:   [16.664 µs 16.732 µs 16.807 µs]
                        change: [-8.0447% -7.6388% -7.2548%] (p = 0.00 < 0.05)
                        Performance has improved.
day07/part2             time:   [19.839 µs 19.957 µs 20.122 µs]
                        change: [+0.3328% +1.5923% +3.2755%] (p = 0.02 < 0.05)
                        Change within noise threshold.

day08/get_input         time:   [193.56 µs 193.68 µs 193.83 µs]
                        change: [-1.7662% -0.6981% +0.3234%] (p = 0.20 > 0.05)
                        No change in performance detected.
day08/part1             time:   [39.988 µs 40.011 µs 40.039 µs]
                        change: [-6.7396% -5.4684% -4.4394%] (p = 0.00 < 0.05)
                        Performance has improved.
day08/part2             time:   [428.86 µs 431.87 µs 435.40 µs]
                        change: [-14.192% -7.0014% -1.1503%] (p = 0.04 < 0.05)
                        Performance has improved.

day09/get_input         time:   [71.766 µs 71.967 µs 72.161 µs]
                        change: [+6.0916% +6.5350% +6.8990%] (p = 0.00 < 0.05)
                        Performance has regressed.
day09/part1             time:   [482.91 µs 484.36 µs 485.81 µs]
                        change: [-1.3131% -0.8624% -0.4138%] (p = 0.00 < 0.05)
                        Change within noise threshold.
day09/part2             time:   [789.27 µs 791.95 µs 795.65 µs]
                        change: [+2.1866% +3.3281% +4.3056%] (p = 0.00 < 0.05)
                        Performance has regressed.

```

## System Allocator Timings

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

day09/get_input         time:   [67.368 µs 67.439 µs 67.533 µs]
day09/part1             time:   [489.55 µs 491.90 µs 494.48 µs]
day09/part2             time:   [766.63 µs 776.74 µs 790.37 µs]

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

day09/get_input         time:   [139.71 µs 143.59 µs 147.98 µs]
day09/part1             time:   [494.95 µs 497.91 µs 501.22 µs]
day09/part2             time:   [735.51 µs 747.46 µs 762.90 µs]

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

## New Allocator Improvments

```
day01/get_input         time:   [45.440 µs 45.478 µs 45.522 µs]
                        change: [+1.9923% +2.9710% +3.8084%] (p = 0.00 < 0.05)
                        Performance has regressed.
day01/part1             time:   [1.1156 ns 1.1197 ns 1.1259 ns]
                        change: [-5.0302% -4.5179% -4.0219%] (p = 0.00 < 0.05)
                        Performance has improved.
day01/part2             time:   [2.6443 ns 2.6662 ns 2.7068 ns]
                        change: [-1.4148% -0.5309% +0.4163%] (p = 0.27 > 0.05)
                        No change in performance detected.

day02/get_input         time:   [51.659 µs 51.809 µs 51.964 µs]
                        change: [-0.1457% +0.4203% +0.9468%] (p = 0.14 > 0.05)
                        No change in performance detected.
day02/part1             time:   [7.8771 µs 7.8896 µs 7.9057 µs]
                        change: [+1.9766% +3.4134% +4.5541%] (p = 0.00 < 0.05)
                        Performance has regressed.
day02/part2             time:   [13.932 µs 13.939 µs 13.947 µs]
                        change: [-8.5967% -7.4716% -6.6586%] (p = 0.00 < 0.05)
                        Performance has improved.

day03/get_input         time:   [25.679 µs 25.760 µs 25.849 µs]
                        change: [-8.1937% -7.5067% -6.9269%] (p = 0.00 < 0.05)
                        Performance has improved.
day03/part1             time:   [498.44 ns 499.34 ns 500.57 ns]
                        change: [+8.0327% +8.9041% +9.6342%] (p = 0.00 < 0.05)
                        Performance has regressed.
day03/part2             time:   [211.04 ns 211.24 ns 211.50 ns]
                        change: [-5.4108% -3.5647% -2.3482%] (p = 0.00 < 0.05)
                        Performance has improved.

day04/get_input         time:   [57.075 µs 57.106 µs 57.140 µs]
                        change: [-9.7980% -8.4254% -7.1591%] (p = 0.00 < 0.05)
                        Performance has improved.
day04/part1             time:   [1.5199 µs 1.5232 µs 1.5274 µs]
                        change: [-21.541% -20.277% -19.240%] (p = 0.00 < 0.05)
                        Performance has improved.
day04/part2             time:   [1.6933 µs 1.6984 µs 1.7041 µs]
                        change: [-17.061% -15.462% -13.940%] (p = 0.00 < 0.05)
                        Performance has improved.

day05/get_input         time:   [18.234 µs 18.269 µs 18.311 µs]
                        change: [-11.246% -9.9724% -8.8016%] (p = 0.00 < 0.05)
                        Performance has improved.
day05/part1             time:   [5.0598 µs 5.0722 µs 5.0885 µs]
                        change: [-18.539% -17.602% -16.782%] (p = 0.00 < 0.05)
                        Performance has improved.
day05/part2             time:   [5.2125 µs 5.2180 µs 5.2247 µs]
                        change: [-20.884% -19.533% -18.229%] (p = 0.00 < 0.05)
                        Performance has improved.

day06/get_input         time:   [1.1336 ns 1.1357 ns 1.1386 ns]
                        change: [-6.8249% -5.6127% -4.5323%] (p = 0.00 < 0.05)
                        Performance has improved.
day06/part1             time:   [3.9257 µs 3.9277 µs 3.9302 µs]
                        change: [-7.1613% -6.1279% -5.1845%] (p = 0.00 < 0.05)
                        Performance has improved.
day06/part2             time:   [5.9778 µs 6.0007 µs 6.0380 µs]
                        change: [-18.250% -12.527% -7.0356%] (p = 0.00 < 0.05)
                        Performance has improved.

day07/get_input         time:   [126.19 µs 126.56 µs 127.04 µs]
                        change: [-6.2881% -5.6931% -4.9627%] (p = 0.00 < 0.05)
                        Performance has improved.
day07/part1             time:   [16.664 µs 16.732 µs 16.807 µs]
                        change: [-8.0447% -7.6388% -7.2548%] (p = 0.00 < 0.05)
                        Performance has improved.
day07/part2             time:   [19.839 µs 19.957 µs 20.122 µs]
                        change: [+0.3328% +1.5923% +3.2755%] (p = 0.02 < 0.05)
                        Change within noise threshold.

day08/get_input         time:   [193.56 µs 193.68 µs 193.83 µs]
                        change: [-1.7662% -0.6981% +0.3234%] (p = 0.20 > 0.05)
                        No change in performance detected.
day08/part1             time:   [39.988 µs 40.011 µs 40.039 µs]
                        change: [-6.7396% -5.4684% -4.4394%] (p = 0.00 < 0.05)
                        Performance has improved.
day08/part2             time:   [428.86 µs 431.87 µs 435.40 µs]
                        change: [-14.192% -7.0014% -1.1503%] (p = 0.04 < 0.05)
                        Performance has improved.

day09/get_input         time:   [71.766 µs 71.967 µs 72.161 µs]
                        change: [+6.0916% +6.5350% +6.8990%] (p = 0.00 < 0.05)
                        Performance has regressed.
day09/part1             time:   [482.91 µs 484.36 µs 485.81 µs]
                        change: [-1.3131% -0.8624% -0.4138%] (p = 0.00 < 0.05)
                        Change within noise threshold.
day09/part2             time:   [789.27 µs 791.95 µs 795.65 µs]
                        change: [+2.1866% +3.3281% +4.3056%] (p = 0.00 < 0.05)
                        Performance has regressed.

```

## System Allocator Timings

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

day09/get_input         time:   [67.368 µs 67.439 µs 67.533 µs]
day09/part1             time:   [489.55 µs 491.90 µs 494.48 µs]
day09/part2             time:   [766.63 µs 776.74 µs 790.37 µs]

```

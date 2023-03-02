# Advent of Code project template
A Rust template for Advent of Code that *someone on Reddit*(sorry, I forgot who it was) made to easily run any day or combination of days and measure the execution time.

Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

To run: `cargo run --release [days...]`

## Downloading Inputs

Get a session cookie from your browser and copy it into `cookie.key`. Then run `./get-input.sh day_you_want_to_download`.

## Benchmark 
on M1 Macbook Pro 13', `rustc 1.69.0-nightly (c18a5e8a5 2023-01-25)`. Input is baked as strings into the binary, hence the time presented do not account for disk access. Though they do account for parsing input. General assumptions about the input format were made, i.e. non-ascii characters, error handling on mal-formed input, etc.

```
=== Day 01 ===
  · Part 1: 66719
  · Part 2: 198551
  · Elapsed: 0.7279 ms

=== Day 02 ===
  · Part 1: 9759
  · Part 2: 12429
  · Elapsed: 1.0700 ms

=== Day 03 ===
  · Part 1: 7826
  · Part 2: 2577
  · Elapsed: 0.2760 ms

=== Day 04 ===
  · Part 1: 431
  · Part 2: 823
  · Elapsed: 0.5472 ms

=== Day 05 ===
  · Part 1: PSNRGBTFT
  · Part 2: BNTZFPMMW
  · Elapsed: 0.8490 ms

=== Day 06 ===
  · Part 1: 1034
  · Part 2: 2472
  · Elapsed: 0.0118 ms

=== Day 07 ===
  · Part 1: 1243729
  · Part 2: 4443914
  · Elapsed: 0.5276 ms

=== Day 08 ===
  · Part 1: 1662
  · Part 2: 537600
  · Elapsed: 0.8313 ms

=== Day 09 ===
  · Part 1: 6011
  · Part 2: 2419
  · Elapsed: 3.7217 ms

=== Day 10 ===
  · Part 1: 13920
  · Part 2: 
####..##..#....#..#.###..#....####...##.
#....#..#.#....#..#.#..#.#....#.......#.
###..#....#....####.###..#....###.....#.
#....#.##.#....#..#.#..#.#....#.......#.
#....#..#.#....#..#.#..#.#....#....#..#.
####..###.####.#..#.###..####.#.....##..
.
  · Elapsed: 0.0279 ms

=== Day 11 ===
  · Part 1: 62491
  · Part 2: 17408399184
  · Elapsed: 8.0345 ms

=== Day 12 ===
  · Part 1: 437
  · Part 2: 430
  · Elapsed: 17.4250 ms

=== Day 13 ===
  · Part 1: 6478
  · Part 2: 21922
  · Elapsed: 2.5963 ms
Total runtime: 36.6461 ms

```

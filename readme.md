# Advent of Code in Rust

I was looking for a simple to use lightweight framework to assist me in solving the upcoming Advent of Code programming puzzles in Rust. I tried [cargo-aoc](https://github.com/gobanos/cargo-aoc/) but it did not work out well for me, so I created my own. To get started just clone this repository.

``` bash
git clone https://github.com/phoony/aoc-scaffold.git
```

## Usage

### Folder Structure

The folder structure is pretty straight forward. For each year of AOC we create a separate directory (1). Said directory contains a days (2) and an inputs (3) subdirectory.

```bash
── src
   ├── aoc2015 (1)
   │   ├── days (2)
   │   │   ├── day01.rs
   │   │   ├── day02.rs
   │   │   ├── ........
   │   │   ├── day25.rs
   │   │   └── mod.rs   <--- # Used to make each day module public, you don't need to touch this
   │   ├── inputs (3)
   │   │   ├── day01.txt
   │   │   ├── day02.txt
   │   │   ├── .........
   │   │   └── day25.txt
   │   └── mod.rs
   ├── common   <--- # Can be used as a library for common code
   │   └── mod.rs
   ├── lib.rs   <--- # New Years have to be made public here
   └── runner.rs <--- # New Years also have to be updated in the runner
```

### Running Solutions

Solutions can be run at any time with the following command:

```bash
cargo run --release -- -y {year} -d {day} -p {part}
```

Year and day are required while part can be omitted. If no part is given then both will be run sequentially.

Example:

``` text
cargo run --release -- -y 2015 -d 1

Running Year 2015 Day 1
------------------------------
Hello From aoc2015::day01::part1()
------------------------------
Elapsed: 112µs

------------------------------
Hello From aoc2015::day01::part2()
------------------------------
Elapsed: 111.4µs
```

### Adding Another Year

I added a template that you can just copy to add another year. Just copy and rename ./template to ./src/aoc{year} to get started with another year. Additionally you have to add `pub mod aoc{year};` to `./src/lib.rs` and add it to the `include_aoc!(year, day, part, aoc2015, ...);` line in `./src/runner.rs`. I know this might be a bit convoluted but you only have to do it [once a year](https://xkcd.com/1205/).

### Files

#### Rust Files

Solutions for each day are stored a corresponding .rs file. The template for each of these files is the same.

```rust
use crate::common::macros::load_input;

load_input!(); // Creates a global variable 'INPUT' which is a lazy static String that can be worked on

fn solve(input: &String) -> i32 {
    42
}

pub fn part1() {
    let solution = solve(&INPUT);
    println!("{}", solution); // prints 42
}

pub fn part2() {}

```

Pretty straightforward, `part1()` is used to implement part 1 of each days puzzle while `part2()` is implements part 2.

#### Input Files

Input files are basic .txt files which contain your puzzle input (you have to fill in the input yourself). Input files get picked up automatically with the `load_input!()` macro. You only have to copy your input to the corresponding day.txt file and the input will be automatically stored in the INPUT global variable.

use aoc::*;

macro_rules! timer {
    ($block:block) => { {
        println!("------------------------------");
        let now = std::time::Instant::now();
        $block
        let elapsed = now.elapsed();
        println!("------------------------------");
        println!("Elapsed: {:?}", elapsed);
    }
    };
}

macro_rules! include_part {
    ($day:path, $part:expr) => {{
        use day::part1;
        use day::part2;
        use $day as day;

        match $part {
            1 => timer!({ part1() }),
            2 => timer!({ part2() }),
            _ => {
                timer!({ part1() });
                println!();
                timer!({ part2() });
            }
        }
    };};
}

macro_rules! include_days {
    ($year:path, $day:expr, $part:expr) => {
        use year::days;
        use $year as year;

        match $day {
            1 => include_part!(days::day01, $part),
            2 => include_part!(days::day02, $part),
            3 => include_part!(days::day03, $part),
            4 => include_part!(days::day04, $part),
            5 => include_part!(days::day05, $part),
            6 => include_part!(days::day06, $part),
            7 => include_part!(days::day07, $part),
            8 => include_part!(days::day08, $part),
            9 => include_part!(days::day09, $part),
            10 => include_part!(days::day10, $part),
            11 => include_part!(days::day11, $part),
            12 => include_part!(days::day12, $part),
            13 => include_part!(days::day13, $part),
            14 => include_part!(days::day14, $part),
            15 => include_part!(days::day15, $part),
            16 => include_part!(days::day16, $part),
            17 => include_part!(days::day17, $part),
            18 => include_part!(days::day18, $part),
            19 => include_part!(days::day19, $part),
            20 => include_part!(days::day20, $part),
            21 => include_part!(days::day21, $part),
            22 => include_part!(days::day22, $part),
            23 => include_part!(days::day23, $part),
            24 => include_part!(days::day24, $part),
            25 => include_part!(days::day25, $part),
            _ => {}
        }
    };
}

macro_rules! include_aoc {
    ($chosen_year:expr, $chosen_day:expr, $chosen_part:expr, $($year:path),*) => {

        $(
            let y = stringify!($year);
            let y = y.strip_prefix("aoc").unwrap();
            let y: i32 = y.parse().unwrap();

            if y == $chosen_year {
                include_days!($year, $chosen_day, $chosen_part);
            }
        )*
    };
}

fn main() {
    let mut year = 2015;
    let mut day = 1;
    let mut part = 0; // defaults to both parts

    let mut arguments = std::env::args();

    while let Some(arg) = arguments.next() {
        if arg.to_lowercase() == "-y" {
            year = arguments
                .next()
                .expect("no year given")
                .parse()
                .expect("could not parse year");
        }

        if arg.to_lowercase() == "-d" {
            day = arguments
                .next()
                .expect("no day given")
                .parse()
                .expect("could not parse day");
        }

        if arg.to_lowercase() == "-p" {
            part = arguments
                .next()
                .expect("no part given")
                .parse()
                .expect("could not parse part");
        }
    }

    match part {
        1 | 2 => {
            println!("Running Year {} Day {} Part {}", year, day, part);
        }
        _ => {
            println!("Running Year {} Day {}", year, day);
        }
    }

    // Do not touch anything above, just add modules here like
    // include_aoc!(year, day, part, aoc2015, aoc2016, aoc2017, ...);
    include_aoc![year, day, part, aoc2015];
}

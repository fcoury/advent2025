fn main() {
    // the number of times the dial is left pointing at 0 after any rotation in the sequence.
    let input = include_str!("./input-test.txt");
    // let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.split('\n').collect();
    println!("{}", count_zero_rotations(&lines));
}

static MAX_DIAL: i32 = 99;
static MIN_DIAL: i32 = 0;

fn count_zero_rotations(movements: &[&str]) -> usize {
    let mut dial = 50;
    let mut zeros = 0;
    for mov in movements {
        if mov.is_empty() {
            continue;
        }

        print!("The dial is rotated {mov} ");
        let mut chars = mov.chars().into_iter();
        let Some(dir) = chars.next() else {
            panic!("Invalid entry: {mov}");
        };

        let clicks = chars.collect::<String>();
        let clicks: i32 = clicks.parse().unwrap();

        let was_zero = dial == 0;

        match dir {
            'R' => dial += clicks,
            'L' => dial -= clicks,
            _ => panic!("Invalid entry: {mov}"),
        }

        let mut pointed_at_zero = false;
        let mut pointed_at_zero_times = 0;
        if dial != 0 {
            // when do we "point at zero"?
            // on the way down = when we are at a positive number and end at a negative number
            // on the way up = when we are at a <= MAX_DIAL and we end up > MAX_DIAL

            loop {
                if dial >= MIN_DIAL && dial <= MAX_DIAL {
                    break;
                }

                if dial > MAX_DIAL {
                    dial = dial - MAX_DIAL - 1;
                }
                if dial < MIN_DIAL {
                    dial = MAX_DIAL + dial + 1;
                }

                pointed_at_zero = true;
                zeros += 1;
            }
        }

        if dial == 0 || was_zero {
            pointed_at_zero = false;
        }

        if pointed_at_zero {
            zeros += 1;
        }

        assert!(dial >= MIN_DIAL, "{dial} NOT >= {MIN_DIAL}");
        assert!(dial <= MAX_DIAL, "{dial} NOT <= {MAX_DIAL}");

        print!("to point at {dial}");

        if pointed_at_zero {
            print!("; during its rotation it points to 0 {pointed_at_zero_times} times");
        }

        println!();

        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}

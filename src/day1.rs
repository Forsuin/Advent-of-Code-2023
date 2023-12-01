use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines()
        .map(|l| {
            let mut first_digit_index = 0;
            let mut last_digit_index = 0;

            for (i, c) in l.chars().enumerate() {
                if c.is_digit(10) {
                    first_digit_index = i;
                    break;
                }
            }

            for (i, c) in l.chars().enumerate() {
                if c.is_digit(10) {
                    if last_digit_index < i {
                        last_digit_index = i;
                    }
                }
            }

            let first = l.chars().nth(first_digit_index).unwrap().to_digit(10).unwrap();
            let second = l.chars().nth(last_digit_index).unwrap().to_digit(10).unwrap();

            let num = (first * 10) + second;

            // let num = format!("{}{}", first, second).parse::<u32>().unwrap();
            num
        }).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let input = input
            .replace("one", "on1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

    input.lines()
        .map(|l| {
            let first = l
                .chars()
                .find(|&c| c > '0' && c <= '9')
                .unwrap()
                .to_digit(10)
                .unwrap() as u32;
            let second = l
                .chars()
                .rfind(|&c| c > '0' && c <= '9')
                .unwrap()
                .to_digit(10)
                .unwrap() as u32;

            first * 10 + second
        }).sum()
}
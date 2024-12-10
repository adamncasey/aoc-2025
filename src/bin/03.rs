advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    println!("{input}");

    let mut sum = 0;
    for (blah, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        println!("{blah} {x}, {y}");
        let (x, y) = (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap());
        sum += x * y;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dos = input.split("do()");

    let mut sum = 0;

    for blah in dos {
        let result = blah.split_once("don't()");

        println!("{result:?}");

        let dos = result.unwrap_or((blah, ""));

        sum += part_one(dos.0).unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}

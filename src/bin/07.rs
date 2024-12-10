advent_of_code::solution!(7);

// too low 1101441272
// too high 6392012777720
// too high 616392012777720
// too high 6392012777720

fn parse_input(input: &str) -> Vec<(u128, Vec<u128>)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let (target, rest) = line.split_once(": ").unwrap();

        parsed.push((
            target.parse::<u128>().unwrap(),
            rest.split(" ")
                .map(|x| x.parse::<u128>().unwrap())
                .collect(),
        ));
    }

    parsed
}

pub fn part_one(input: &str) -> Option<u128> {
    let input = parse_input(input);

    let mut sum = 0;

    for mut row in input {
        println!("{row:?}");
        if it_works(row.0, &mut row.1, 0, 0, false) {
            sum += row.0;
        }
    }

    //println!("{sum}");

    Some(sum as u128)
}

pub fn part_two(input: &str) -> Option<u128> {
    let input = parse_input(input);

    let mut sum = 0;

    for mut row in input {
        //print!("{row:?}");
        if it_works(row.0, &mut row.1, 0, 0, true) {
            sum += row.0;
            //print!(" works");
        }
        //println!("");
    }

    //println!("{sum}");

    Some(sum as u128)
}

fn it_works(target: u128, nums: &mut [u128], index: usize, so_far: u128, extra_op: bool) -> bool {
    if so_far > target {
        return false;
    }
    if index >= (nums.len()) {
        let result = so_far == target;
        //println!(" = {result}");
        return so_far == target;
    }

    //print!("+ {}", nums[index]);
    if !it_works(target, nums, index + 1, so_far + nums[index], extra_op) {
        //print!("* {}", nums[index]);

        let mul_target = if so_far == 0 {
            nums[index]
        } else {
            so_far * nums[index]
        };

        if !it_works(target, nums, index + 1, mul_target, extra_op) {
            if !extra_op || so_far == 0 {
                return false;
            }

            // concat this number with so_far if we have another index after this
            let new_num = concat(so_far, nums[index]);

            let result = it_works(target, nums, index + 1, new_num, extra_op);
            if !result {
                return false;
            } else {
                //println!("{} never here?", nums[index + 1]);
            }
        }
    }

    //print!("+ {}", nums[index]);

    true
}

fn concat(x: u128, y: u128) -> u128 {
    x * (10_u128.pow(y.ilog10() + 1)) + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_test() {
        assert_eq!(concat(10, 1), 101);
        assert_eq!(concat(1000, 1), 10001);
        assert_eq!(concat(15, 6), 156);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_one_1() {
        let input = "583: 2 14 2 6 20";

        let mut input = &mut parse_input(input)[0];

        assert!(!it_works(input.0, &mut input.1, 0, 0, false));

        let input = "77: 71 5 1";
        let input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, false));

        let input = "76: 71 5";
        let input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, false));

        let input = "50: 10 5";
        let input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, false));

        let input = "15: 10 5";
        let mut input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, false));

        let input = "15: 15";
        let mut input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, false));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_part_two_2() {
        let input = "83: 17 5";
        let mut input = &mut parse_input(input)[0];

        //assert!(!it_works(input.0, &mut input.1, 0, 0, true));

        let input = "156: 15 6";
        let input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, true));

        let input = "7290: 6 8 6 15";
        let input = &mut parse_input(input)[0];

        assert!(it_works(input.0, &mut input.1, 0, 0, true));

        let input = "192: 17 8 14";
        let mut input = &mut parse_input(input)[0];

        //assert!(it_works(input.0, &mut input.1, 0, 0, true));
    }
}

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut count = 0;

    for report in lines {
        if report_is_safe(report, true) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut count = 0;

    for report in lines {
        if report_is_safe(report, false) {
            count += 1;
            println!("true {report}");
        } else {
            println!("false {report}");
        }
    }

    Some(count)
}

fn rest_of_report_safe(
    nums: &[u32],
    index: usize,
    prev: u32,
    direction: Option<bool>,
    skipped_already: bool,
) -> bool {
    let num = *match nums.get(index) {
        None => {
            //print!("-> true\n");
            return true;
        }
        Some(num) => num,
    };

    //print!("({prev} {num}) ");
    let new_direction = match direction {
        Some(direction) => {
            if (num > prev) != direction {
                //print!(".. -> false (direction swap)\n");
                return false;
            }

            Some(direction)
        }
        None => {
            if num > prev {
                Some(true)
            } else if num < prev {
                Some(false)
            } else {
                //print!(".. -> false (same number)\n");
                return false;
            }
        }
    };

    let diff = num.abs_diff(prev);
    if diff > 3 || diff < 1 {
        //print!(".. -> false (gap nonexistent or too large)\n");
        return false;
    }

    if !rest_of_report_safe(nums, index + 1, num, new_direction, skipped_already) {
        if skipped_already {
            return false;
        }
        //print!("Try skipping next...");
        if !rest_of_report_safe(nums, index + 2, num, new_direction, true) {
            return false;
        }
    }

    return true;
}

fn report_is_safe(report: &str, already_skipped: bool) -> bool {
    let nums: Vec<u32> = report
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    if nums.len() == 0 {
        panic!("Empty report");
    }

    let prev = *nums
        .get(0)
        .expect("Report must have at least one number in {report:?}");

    //print!("{prev} ");
    // if set, next number needs to be higher than previous
    let direction: Option<bool> = None;

    if rest_of_report_safe(&nums, 1, prev, direction, already_skipped) {
        return true;
    }

    if already_skipped {
        return false;
    }

    let new_prev = *nums.get(1).expect("Reports have more than two numbers");

    rest_of_report_safe(&nums, 2, new_prev, direction, true)
        || rest_of_report_safe(&nums, 2, prev, direction, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let nums = "1 2 3 4 5 6 7";
        assert!(report_is_safe(nums, false));

        let nums = "9 6 12";
        assert!(report_is_safe(nums, false));

        let nums = "8 9 6 12";
        assert!(report_is_safe(nums, false));

        let nums = "1 2 3 4 5 6 15";
        assert!(report_is_safe(nums, false));

        let nums = "1 2 3 15 5 6 7";
        assert!(report_is_safe(nums, false));

        let nums = "1 2 3 4 5 0 6";
        assert!(report_is_safe(nums, false));

        let nums = "1 2 3 4 5 3 6";
        assert!(report_is_safe(nums, false));

        let nums = "1 0 1 2 3 4 5";
        assert!(report_is_safe(nums, false));

        let nums = "1 0 1 2 3 4 5";
        assert!(report_is_safe(nums, false));

        let nums = "7 0 1 2 3 4 5";
        assert!(report_is_safe(nums, false));

        let nums = "5 6 4 7 8 9";
        assert!(report_is_safe(nums, false));

        let nums = "5 4 6 7 8 9";
        assert!(report_is_safe(nums, false));

        let nums = "9 8 7 10 6 5";
        assert!(report_is_safe(nums, false));

        let nums = "9 8 7 10 6 11";
        assert!(!report_is_safe(nums, false));

        let nums = "9 8 7 10 6 11";
        assert!(!report_is_safe(nums, false));

        let nums = "9 6 12";
        assert!(report_is_safe(nums, false));

        let nums = "8 9 6 12 13 15";
        assert!(report_is_safe(nums, false));
        let nums = "9 6 12 13 15";
        assert!(report_is_safe(nums, false));
    }
}

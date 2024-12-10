use core::num;

use itertools::Itertools;

advent_of_code::solution!(4);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut num_xmas = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            num_xmas += how_many_xmas(&grid, (x as i32, y as i32));
        }
    }

    Some(num_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut num_cross_mas = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_cross_mas(&grid, (x as i32, y as i32)) {
                num_cross_mas += 1;
            }
        }
    }

    Some(num_cross_mas)
}

fn how_many_xmas(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> u32 {
    let directions: Vec<(i32, i32)> = (-1..=1).cartesian_product(-1..=1).collect();
    println!("{directions:?}");

    let mut sum = 0;
    for direction in directions {
        if direction == (0, 0) {
            continue;
        }

        if is_blah("XMAS", grid, pos, direction) {
            sum += 1
        }
    }

    sum
}

fn is_cross_mas(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    if grid[pos.1 as usize][pos.0 as usize] != 'A' {
        return false;
    }

    let top_left_pos = (pos.0 - 1, pos.1 - 1);
    let top_left_dir = (1, 1);

    let bottom_left_pos = (pos.0 - 1, pos.1 + 1);
    let bottom_left_dir = (1, -1);

    (is_blah("MAS", grid, top_left_pos, top_left_dir)
        || is_blah("SAM", grid, top_left_pos, top_left_dir))
        && (is_blah("MAS", grid, bottom_left_pos, bottom_left_dir)
            || is_blah("SAM", grid, bottom_left_pos, bottom_left_dir))
}

fn is_blah(
    search: &str,
    grid: &Vec<Vec<char>>,
    mut pos: (i32, i32),
    direction: (i32, i32),
) -> bool {
    for ch in search.chars() {
        if pos.0 < 0
            || pos.1 < 0
            || pos.1 as usize >= grid.len()
            || pos.0 as usize >= grid[pos.1 as usize].len()
        {
            return false;
        }

        if grid[pos.1 as usize][pos.0 as usize] != ch {
            return false;
        }

        pos = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);
    }

    println!("Found {search} ending at {pos:?} {direction:?}");

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

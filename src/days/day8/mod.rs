#[derive(Clone, Copy, Debug)]
struct Vec2D(isize, isize);

struct GridIterator<'a> {
    grid: &'a Vec<Vec<u32>>,
    cords: Vec2D,
    direction: Vec2D,
}

fn make_directional_grid_iter<'a>(
    grid: &'a Vec<Vec<u32>>,
    cords: Vec2D,
    direction: Vec2D,
) -> GridIterator<'a> {
    GridIterator {
        grid,
        cords,
        direction,
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (&'a u32, Vec2D);

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .grid
            .get(self.cords.0 as usize)
            .map(|r| r.get(self.cords.1 as usize))
        {
            Some(Some(v)) => {
                let val = Some((v, self.cords));
                self.cords = self.cords + self.direction;
                val
            }
            _ => None,
        }
    }
}

impl std::ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn check_direction(grid: &Vec<Vec<u32>>, direction: Vec2D, start: Vec2D) -> bool {
    let inital_val = grid[start.0 as usize][start.1 as usize];
    for (&val, _) in make_directional_grid_iter(grid, start + direction, direction) {
        if val >= inital_val {
            return false;
        }
    }
    return true;
}
fn check_score(grid: &Vec<Vec<u32>>, direction: Vec2D, start: Vec2D) -> u32 {
    let inital_val = grid[start.0 as usize][start.1 as usize];
    let mut count = 0;
    for (&val, _) in make_directional_grid_iter(grid, start + direction, direction) {
        if val >= inital_val {
            return count + 1;
        }
        count += 1;
    }
    return count;
}

pub fn part_1(input: &str) -> usize {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut counter = 0;
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[i].len() - 1) {
            if check_direction(&grid, Vec2D(0, 1), Vec2D(i as isize, j as isize))
                || check_direction(&grid, Vec2D(1, 0), Vec2D(i as isize, j as isize))
                || check_direction(&grid, Vec2D(0, -1), Vec2D(i as isize, j as isize))
                || check_direction(&grid, Vec2D(-1, 0), Vec2D(i as isize, j as isize))
            {
                counter += 1
            }
        }
    }
    return counter + (grid.len() * 4) - 4;
}
pub fn part_2(input: &str) -> u32 {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();
    let mut best_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let score = check_score(&grid, Vec2D(0, 1), Vec2D(i as isize, j as isize))
                * check_score(&grid, Vec2D(1, 0), Vec2D(i as isize, j as isize))
                * check_score(&grid, Vec2D(0, -1), Vec2D(i as isize, j as isize))
                * check_score(&grid, Vec2D(-1, 0), Vec2D(i as isize, j as isize));
            best_score = std::cmp::max(score, best_score);
        }
    }
    return best_score;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 21);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 8);
    }
}

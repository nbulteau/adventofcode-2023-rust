 use std::collections::HashSet;

pub fn part_one(data: &str) -> usize {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let start_point = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, &cell)| {
            if cell == 'S' {
                Some((i, j))
            } else {
                None
            }
        })
    }).unwrap();
    let path = get_path(start_point, &grid);

    path.len() / 2
}

 pub fn part_two(data: &str) -> usize {
     let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
     let start_point = grid.iter().enumerate().find_map(|(i, row)| {
         row.iter().enumerate().find_map(|(j, &cell)| {
             if cell == 'S' {
                 Some((i, j))
             } else {
                 None
             }
         })
     }).unwrap();
     let path = get_path(start_point, &grid);

     let mut in_points = HashSet::new();
     for (i, row) in grid.iter().enumerate() {
         for (j, _) in row.iter().enumerate() {
             let point = (i, j);
             if !path.contains(&point) {
                 let mut windings = 0;
                 for y in (0..=j).rev() {
                     let point_to_test = (i, y);
                     if "|JL".contains(grid[point_to_test.0][point_to_test.1]) && path.contains(&point_to_test) {
                         windings += 1;
                     }
                 }
                 if windings % 2 == 1 {
                     in_points.insert(point);
                 }
             }
         }
     }

     // debug
     // clean_grid_and_display(&mut grid.clone(), &path, &in_points);

     in_points.len()
 }

 fn get_path(start_point: (usize, usize), grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
     let mut visited = HashSet::new();
     visited.insert(start_point);

     let mut to_explore = vec![(start_point, 0)];
     while let Some((current_point, distance)) = to_explore.pop() {
         for neighbor in get_cardinal_neighbors(current_point, grid) {
             if to_explore.iter().any(|(point, _)| *point == neighbor) {
                 continue;
             }
             if is_connecting(current_point, neighbor, grid) && !visited.contains(&neighbor) {
                 visited.insert(neighbor);
                 to_explore.push((neighbor, distance + 1));
             }
         }
     }

     visited
 }

 fn get_cardinal_neighbors(point: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
     let mut neighbors = Vec::new();
     let (x, y) = point;
     let max_x = grid.len() - 1;
     let max_y = grid[0].len() - 1;

     if x > 0 {
         neighbors.push((x - 1, y));
     }
     if x < max_x {
         neighbors.push((x + 1, y));
     }
     if y > 0 {
         neighbors.push((x, y - 1));
     }
     if y < max_y {
         neighbors.push((x, y + 1));
     }

     neighbors
 }

 fn is_connecting(first_point: (usize, usize), second_point: (usize, usize), grid: &Vec<Vec<char>>) -> bool {
     // Get the current character value and the neighbor character value from the grid
     let current = grid[first_point.0][first_point.1];
     let neighbor = grid[second_point.0][second_point.1];
     // Get the delta between the two points
     let dx = second_point.0 as i32 - first_point.0 as i32;
     let dy = second_point.1 as i32 - first_point.1 as i32;
     // up, down, left, right
     let up = dx == -1 && dy == 0;
     let down = dx == 1 && dy == 0;
     let left = dx == 0 && dy == -1;
     let right = dx == 0 && dy == 1;

     // Return whether the current character can connect to the neighbor character in the given direction
     match current {
         'S' => (neighbor == '|' || neighbor == '7' || neighbor == 'F') && up || (neighbor == '|' || neighbor == 'L' || neighbor == 'J') && down || (neighbor == '-' || neighbor == 'J' || neighbor == '7') && right || (neighbor == '-' || neighbor == 'F' || neighbor == 'L') && left,
         '|' => (neighbor == '|' || neighbor == 'L' || neighbor == 'J') && down || (neighbor == '|' || neighbor == 'F' || neighbor == '7') && up,
         '-' => (neighbor == '-' || neighbor == 'F' || neighbor == 'L') && left || (neighbor == '-' || neighbor == 'J' || neighbor == '7') && right,
         'J' => (neighbor == '|' || neighbor == '7' || neighbor == 'F') && up || (neighbor == '-' || neighbor == 'L' || neighbor == 'F') && left,
         'F' => (neighbor == '|' || neighbor == 'J' || neighbor == 'L') && down || (neighbor == '-' || neighbor == '7' || neighbor == 'J') && right,
         '7' => (neighbor == '|' || neighbor == 'J' || neighbor == 'L') && down || (neighbor == '-' || neighbor == 'F' || neighbor == 'L') && left,
         'L' => (neighbor == '|' || neighbor == '7' || neighbor == 'F') && up || (neighbor == '-' || neighbor == 'J' || neighbor == '7') && right,
         _ => false,
     }
 }
/*
 // Debug functions
 fn clean_grid_and_display(grid: &mut Vec<Vec<char>>, path: &HashSet<(usize, usize)>, in_points: &HashSet<(usize, usize)>) {
     for (i, row) in grid.iter_mut().enumerate() {
         for (j, cell) in row.iter_mut().enumerate() {
             let point = (i, j);
             if in_points.contains(&point) {
                 *cell = 'I';
             } else if !path.contains(&point) {
                 *cell = '.';
             } else {
                 match *cell {
                     'S' => *cell = 'S',
                     'J' => *cell = '┘',
                     'F' => *cell = '┌',
                     '7' => *cell = '┐',
                     'L' => *cell = '└',
                     _ => (),
                 }
             }
         }
     }
     display(grid);
 }

 fn display(grid: &Vec<Vec<char>>) {
     for row in grid {
         for &cell in row {
             print!("{}", cell);
         }
         println!();
     }
 }
*/

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part_one_examples() {
        let training: &str = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        "};

        assert_eq!(4, part_one(training));
    }

    #[test]
    fn part_two_examples() {
        let training: &str = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};

        assert_eq!(8, part_two(training));
    }
}




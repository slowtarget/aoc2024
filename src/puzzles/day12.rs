use std::collections::HashSet;

// Define a structure to hold the perimeter
#[derive(Debug)]

struct Perimeter {
    boundary_segments: Vec<(bool, (isize, isize), (isize, isize), (isize, isize))>, // List of boundary segments
}

impl Perimeter {
    fn new() -> Self {
        Perimeter {
            boundary_segments: Vec::new(),
        }
    }

    // Add a segment to the perimeter
    //   if the y's of start and end are equal then its vertical!!
    //

    fn add_segment(&mut self, inner: (isize, isize), outer: (isize, isize)) {
        let is_horizontal = outer.0 == inner.0;

        let rank = if is_horizontal { (inner.1,outer.1) } else { (inner.0, outer.0) };
        let segment = (is_horizontal, rank, inner, outer);
        self.boundary_segments            .push(segment);
    }
}

// Parse the input into a grid of characters
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// Check if a cell is within the grid bounds
fn is_in_bounds(x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    y < grid.len() && x < grid[0].len()
}

// Flood-fill algorithm to calculate area and perimeter
fn flood_fill(
    x: usize,
    y: usize,
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    plant_type: char,
) -> (usize, Perimeter) {
    let mut stack = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = Perimeter::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // (dy, dx) Right, Down, Left, Up

    while let Some((cx, cy)) = stack.pop() {
        if !visited.insert((cx, cy)) {
            continue;
        }

        area += 1;

        for &(dy, dx) in &directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0
                && ny >= 0
                && is_in_bounds(nx as usize, ny as usize, grid)
                && grid[ny as usize][nx as usize] == plant_type
            {
                if !visited.contains(&(nx as usize, ny as usize)) {
                    stack.push((nx as usize, ny as usize));
                }
            } else {
                // Add a boundary segment for this direction
                let inner = (cx as isize, cy as isize);
                let outer = (cx as isize + dx, cy as isize + dy);
                perimeter.add_segment((inner.0, inner.1), (outer.0, outer.1));
            }
        }
    }

    (area, perimeter)
}

// Calculate the number of sides (continuous edges) from the perimeter
fn calculate_sides(perimeter: &Perimeter) -> usize {
    let mut sides = 0;
    let debug = false;
    // Sort segments by direction and coordinates
    let mut sorted_segments = perimeter.boundary_segments.clone();
    sorted_segments.sort_by(|a, b| a.cmp(b));
    if debug {
        println!("sorted: {:?}", sorted_segments);
    }
    // Group and count continuous segments


    let mut previous: std::option::Option<(bool, (isize, isize), (isize, isize), (isize, isize))> = None;
    // the perimeters are described by the coordinates of the two cells either side of the perimeter line
    //    two continuous vertical edges will have
    //     startA.0 == startB.0 && startA.1 + 1 == startB.1
    //    two continuous horizontal edges will have
    //     startA.1 == startB.1 && startA.0 + 1 == startB.0
    // so a continuous horizontal line will look like this:
    // (0, 0), (0, 1)
    // (1, 0), (1, 1)
    // (2, 0), (2, 1)
    // so compare the previous start with the current start and if the difference in the coords is 1 - then they are continuous
    for &segment in &sorted_segments {
        let (is_horizontal, rank, cell1, cell2) = segment;
        if let Some((previous_direction, previous_rank, previous_inner, previous_outer)) = previous {
            // we have a previous
            let diff = cell1.0 - previous_inner.0 + cell1.1 - previous_inner.1;
            if previous_direction == is_horizontal 
                && rank == previous_rank
                && diff == 1
                && (is_horizontal && cell1.1 == previous_inner.1
                    || !is_horizontal && cell1.0 == previous_inner.0)
            {
                // its going in the same direction and continuous
                // do nothing
                if debug {
                    println!(
                        "span continues {:?}",
                        (
                            previous_direction,
                            is_horizontal,
                            previous_inner,
                            previous_outer,
                            cell1,
                            cell2
                        )
                    );
                }
            } else {
                // add to sides
                sides += 1;
            }
        }
        previous = Some(segment);
    }

    // Count the last group as a side
    if previous.is_some() {
        sides += 1;
    }

    sides
}

// Main function to solve the problem for both parts
pub(crate) fn solve(input: &str) -> (usize, usize) {
    let grid = parse_input(input);
    let mut visited = HashSet::new();
    let mut total_cost_part1 = 0;
    let mut total_cost_part2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                let plant_type = grid[y][x];
                let (area, perimeter) = flood_fill(x, y, &grid, &mut visited, plant_type);

                // Calculate Part 1
                total_cost_part1 += area * perimeter.boundary_segments.len();

                // Calculate Part 2
                let sides = calculate_sides(&perimeter);
                // println!(
                //     "plant type: {}, area: {}, sides: {}",
                //     grid[y][x], area, sides
                // );
                total_cost_part2 += area * sides;
            }
        }
    }
    // assert!(total_cost_part2 > 835484);
    (total_cost_part1, total_cost_part2) //  part2 : 835484 your answer is too low
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;
    mod parse_input {
        use super::*;
        #[test]
        fn test_aa() {
            let grid = parse_input("aa");
            assert_eq!(grid, vec![vec!['a', 'a']]);
            assert_eq!(grid[0][0], 'a');
            assert_eq!(grid[0][1], 'a');
        }
    }
    mod test_flood_fill {
        use super::*;
        #[test]
        fn test_flood_fill_single_cell() {
            let input = vec![vec!['A']];
            let mut visited = HashSet::new();
            let (area, perimeter) = flood_fill(0, 0, &input, &mut visited, 'A');
            let expected = [
                (false, (0,1), (0, 0), (1, 0)),
                (true, (0,1), (0, 0), (0, 1)),
                (false, (0,-1), (0, 0), (-1, 0)),
                (true, (0,-1), (0, 0), (0, -1)),
            ];

            assert_eq!(area, 1); // One cell
            assert_eq!(perimeter.boundary_segments, expected); // Four edges
            assert_eq!(perimeter.boundary_segments.len(), 4);
        }

        #[test]
        fn test_flood_fill_aa() {
            let input = vec![vec!['A', 'A']];
            let mut visited = HashSet::new();
            let (area, actual) = flood_fill(0, 0, &input, &mut visited, 'A');
            //    -------------------------------------
            //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
            //    -------------------------------------
            //    |  -1,0   |  0,0 * |  1,0 * | 2,0   |
            //   --------------------------------------
            //    |  -1,1   |  0,1   |  1,1   | 2,1   |
            //    -------------------------------------

            let expected = [
                (false, (0, -1)  , (0, 0), (-1, 0)), // left
                (false, (1, 2 )  , (1, 0), (2, 0)),   // right
                (true,  (0, -1)  , (0, 0), (0, -1)),  // top left
                (true,  (0, -1)  , (1, 0), (1, -1)),  // top right
                (true,  (0, 1 )  , (0, 0), (0, 1)),    // bottom left
                (true,  (0, 1 )  , (1, 0), (1, 1)),    // bottom right
            ];

            let mut sorted_segments = actual.boundary_segments.clone();
            sorted_segments.sort_by(|a, b| a.cmp(b));

            assert_eq!(area, 2); // Two cells
            assert_eq!(sorted_segments, expected); // Four edges

            assert_eq!(actual.boundary_segments.len(), 6); // Six edges (shared edge counts once)
        }

        #[test]
        fn test_flood_fill_block_of_2x2() {
            //    -------------------------------------
            //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
            //    -------------------------------------
            //    |  -1,0   |  0,0 * |  1,0 * | 2,0   |
            //   --------------------------------------
            //    |  -1,1   |  0,1 * |  1,1 * | 2,1   |
            //    -------------------------------------
            //    |  -1,2  |  0,2    |  1,2   | 2,2   |
            //    -------------------------------------
            let input = vec![vec!['A', 'A'], vec!['A', 'A']];
            let mut visited = HashSet::new();
            let (area, perimeter) = flood_fill(0, 0, &input, &mut visited, 'A');
            let expected = [
                (false,  (0, -1 )   , (0, 0), (-1, 0)), //left top
                (false,  (0, -1 )   , (0, 1), (-1, 1)), // left bottom
                (false,  (1, 2  )   , (1, 0), (2, 0)),   // right top
                (false,  (1, 2  )   , (1, 1), (2, 1)),   // right bottom
                (true,   (0, -1 )   , (0, 0), (0, -1)),  // top left
                (true,   (0, -1 )   , (1, 0), (1, -1)),  // top right
                (true,   (1, 2  )   , (0, 1), (0, 2)),    // bottom left
                (true,   (1, 2  )   , (1, 1), (1, 2)),    // bottom right
            ];
            let mut actual = perimeter.boundary_segments.clone();
            actual.sort();
            println!("actual {:?}", perimeter.boundary_segments);
            println!("sorted {:?}", actual);

            assert_eq!(area, 4); // Four cells
            assert_eq!(actual, expected); // Four edges
            assert_eq!(perimeter.boundary_segments.len(), 8); // Four sides of the square
        }
    }
    mod calculate_sides {
        use super::*;

        #[test]
        fn test_calculate_sides_single_cell() {
            let mut perimeter = Perimeter::new();

            //    -------------------------------------
            //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
            //    -------------------------------------
            //    |  -1,0   |  0,0   |  1,0   | 2,0   |
            //   --------------------------------------
            //    |  -1,1   |  0,1   |  1,1   | 2,1   |
            //    -------------------------------------
            //   if the y's of start and end are equal then its vertical!!
            //
            //    two continuous vertical edges will have
            //     startA.0 == startB.0 && startA.1 + 1 == startB.1
            //    two continuous horizontal edges will have
            //     startA.1 == startB.1 && startA.0 + 1 == startB.0
            //   so for cell 0,0 its perimeters are:
            //    left: (-1, 0), (0, 0)
            //    right: (0, 0), (1, 0)
            //    top: (0, 0), (0, 1)
            //    bottom: (0, -1), (0, 0)
            //
            perimeter.add_segment((0, 0), (0, 1)); // Bottom
            perimeter.add_segment((0, 0), (1, 0)); // Right
            perimeter.add_segment((0, -1), (0, 0)); // Top
            perimeter.add_segment((-1, 0), (0, 0)); // Left

            assert_eq!(calculate_sides(&perimeter), 4); // Four distinct sides
        }
        #[test]
        fn test_calculate_sides_three_edges() {
            let boundary_segments = [
                ((0, 0), (0, 1)), // Horizontal
                ((1, 0), (1, 1)), // Horizontal
                ((0, 2), (1, 2)), // Vertical
                ((1, 2), (1, 1)), // Horizontal
                ((2, 2), (2, 1)), // Horizontal
            ];
            let mut perimeter = Perimeter::new();
            boundary_segments
                .iter()
                .for_each(|(start, end)| perimeter.add_segment(*start, *end));

            assert_eq!(calculate_sides(&perimeter), 3); // Four distinct sides
        }

        #[test]
        fn test_calculate_sides_single_edge() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, 0), (0, 1));
            assert_eq!(calculate_sides(&perimeter), 1);
        }
        #[test]
        fn test_calculate_sides_two_edges_one_side() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, 0), (0, 1));
            perimeter.add_segment((1, 0), (1, 1));
            assert_eq!(calculate_sides(&perimeter), 1);
        }
        #[test]
        fn test_calculate_sides_three_edges_one_side() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, 0), (1, 0));
            perimeter.add_segment((0, 1), (1, 1));
            perimeter.add_segment((0, 2), (1, 2));
            assert_eq!(calculate_sides(&perimeter), 1);
        }
        #[test]
        fn test_calculate_sides_two_edges_two_sides() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, 0), (0, 1));
            perimeter.add_segment((1, 1), (1, 2));
            assert_eq!(calculate_sides(&perimeter), 2);
        }
        #[test]
        fn test_calculate_sides_two_edges_two_sides_not_continuous() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, 0), (0, 1));
            perimeter.add_segment((0, 2), (0, 3));
            assert_eq!(calculate_sides(&perimeter), 2);
        }
        //    -------------------------------------
        //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
        //    -------------------------------------
        //    |  -1,0   |  0,0 * |  1,0 * | 2,0   |
        //   --------------------------------------
        //    |  -1,1   |  0,1   |  1,1   | 2,1   |
        //    -------------------------------------
        #[test]
        fn test_calculate_sides_two_adjacent_cells() {
            let mut perimeter = Perimeter::new();
            perimeter.add_segment((0, -1), (0, 0)); // Top-left
            perimeter.add_segment((1, -1), (1, 0)); // Top-right
            perimeter.add_segment((1, 0), (2, 0)); // Right
            perimeter.add_segment((0, 0), (0, 1)); // Bottom-right
            perimeter.add_segment((1, 0), (1, 1)); // Bottom-left
            perimeter.add_segment((-1, 0), (0, 0)); // Left

            assert_eq!(calculate_sides(&perimeter), 4); // Single rectangle
        }

        #[test]
        fn test_calculate_sides_l_shape() {
            //    -------------------------------------
            //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
            //    -------------------------------------
            //    |  -1,0   |  0,0 * |  1,0   | 2,0   |
            //   --------------------------------------
            //    |  -1,1   |  0,1 * |  1,1 * | 2,1   |
            //    -------------------------------------
            //    |  -1,2   |  0,2   |  1,2   | 2,2   |
            //    -------------------------------------

            let mut perimeter = Perimeter::new();
            // Top
            perimeter.add_segment((0, -1), (0, 0));
            // Left
            perimeter.add_segment((-1, 0), (0, 0));
            perimeter.add_segment((-1, 1), (0, 1));
            // Bottom
            perimeter.add_segment((0, 1), (0, 2));
            perimeter.add_segment((1, 1), (1, 2));
            // far right
            perimeter.add_segment((1, 1), (2, 1));
            // top of leg
            perimeter.add_segment((1, 0), (1, 1));
            // right of top block
            perimeter.add_segment((0, 0), (1, 0));

            assert_eq!(calculate_sides(&perimeter), 6); // L-shape has 6 sides
        }

        #[test]
        fn test_aa() {
            //    -------------------------------------
            //    |  -1,-1  |  0,-1  |  1,-1  | 2,-1  |
            //    -------------------------------------
            //    |  -1,0   |  0,0 * |  1,0 * | 2,0   |
            //   --------------------------------------
            //    |  -1,1   |  0,1   |  1,1   | 2,1   |
            //    -------------------------------------

            let segments =             [
                (false,  (0, -1), (0, 0), (-1, 0)), // left
                (false,  (1, 2 ), (1, 0), (2, 0)),   // right
                (true,   (0, -1), (0, 0), (0, -1)),  // top left
                (true,   (0, -1), (1, 0), (1, -1)),  // top right
                (true,   (0, 1 ), (0, 0), (0, 1)),    // bottom left
                (true,   (0, 1 ), (1, 0), (1, 1)),    // bottom right
            ];
            let mut perimeter = Perimeter::new();
            for segment in segments.iter() {
                perimeter.add_segment(segment.2, segment.3);
            }
            assert_eq!(perimeter.boundary_segments, segments);
            assert_eq!(calculate_sides(&perimeter), 4);
        }
    }
    mod integration {
        use super::*;
        #[test]
        fn test_example_abcde() {
            let input = "AAAA\nBBCD\nBBCC\nEEEC";
            assert_eq!(solve(input), (140, 80)); // Part 1 = 140, Part 2 = 80
        }

        #[test]
        fn test_example_xo() {
            let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
            assert_eq!(solve(input), (772, 436)); // Part 1 = 772, Part 2 = 436
        }

        #[test]
        fn test_large_example() {
            let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
            assert_eq!(solve(input), (1930, 1206)); // Part 1 = 1930, Part 2 = 1206
        }

        #[test]
        fn test_example_1a() {
            let input = "A";
            assert_eq!(solve(input), (4, 4)); // Part 1 = 4, Part 2 = 4
        }

        #[test]
        fn test_example_pair() {
            let input = "AA";
            assert_eq!(solve(input), (12, 8)); // Part 1 = 12, Part 2 = 8
        }

        #[test]
        fn test_l_shape() {
            let input = "Abb\nAAb";
            assert_eq!(solve(input), (48, 36)); // Part 1 = 48, Part 2 = 36
        }

        #[test]
        fn test_square_with_hole() {
            let input = "AAA\nA A\nAAA";
            assert_eq!(solve(input), (132, 68)); // Part 1 = 16, Part 2 = 64
        }
        #[test]
        fn test_square_e() {
            let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
            assert_eq!(solve(input), (692, 236)); // Part 1 = 16, Part 2 = 64
        }
        #[test]
        fn test_square_ab() {
            let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
            assert_eq!(solve(input), (1184, 368)); // Part 1 = 16, Part 2 = 64
        }
    }
}

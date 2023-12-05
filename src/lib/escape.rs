use std::collections::VecDeque;
use std::iter::zip;

type Matrix = Vec<Vec<usize>>;
const DIRECTIONS: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

pub fn find_escape_route(map: Matrix) -> usize {
    // count steps from both the entrance and the exit
    let step_count_map_from_entrance = create_step_count_map(&map);
    let step_count_map_from_exit = flip_map(create_step_count_map(&flip_map(map)));

    // if there is a shortest path (even with a wall to break),
    // the shortest path should pass the point that has the minimum step counts when counting from the entrance and the exit
    zip(step_count_map_from_entrance, step_count_map_from_exit)
        .map(|rows| {
            zip(rows.0, rows.1)
                // reach the same position from both entrance and exit, actual step counts equal to SUM - 1
                .map(|values| values.0 + values.1 - 1)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn create_step_count_map(map: &Matrix) -> Matrix {
    let map_size = map.len();
    let max_steps = map_size * map_size;

    // create a matrix to record the steps between any point in maze and the entrance point
    // all positions are set to unvisited except for the entrance
    let mut step_count_map: Matrix = vec![vec![max_steps; map_size]; map_size];
    step_count_map[0][0] = 1;

    // add the entrance into the traverse queue (Breadth First Search)
    let mut step_queue: VecDeque<(isize, isize)> = VecDeque::new();
    step_queue.push_back((0, 0));

    while !step_queue.is_empty() {
        if let Some((x, y)) = step_queue.pop_front() {
            for direction in DIRECTIONS {
                let (next_x, next_y) = (x + direction[0], y + direction[1]);

                // if next step is within the map and not visited
                if (0..map_size as isize).contains(&next_x)
                    && (0..map_size as isize).contains(&next_y)
                    && step_count_map[next_x as usize][next_y as usize] == max_steps
                {
                    // because removing 1 wall is acceptable
                    // 4 directions from current position can be seen as reachable and will take 1 step
                    step_count_map[next_x as usize][next_y as usize] =
                        step_count_map[x as usize][y as usize] + 1;

                    // if the next step will not get into wall, then add to the traverse queue
                    if map[next_x as usize][next_y as usize] == 0 {
                        step_queue.push_back((next_x, next_y));
                    }
                }
            }
        }
    }

    step_count_map
}

fn flip_map(map: Matrix) -> Matrix {
    // flip the map, so the exit will become the entrance
    // this make it easy to count steps when the map is not square shape
    map.into_iter()
        .rev()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_escape_route() {
        assert_eq!(find_escape_route(vec![vec![0, 1], vec![1, 0]]), 3);

        assert_eq!(
            find_escape_route(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 1, 0, 0],
                vec![1, 1, 1, 0],
            ]),
            7
        );

        assert_eq!(
            find_escape_route(vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1],
                vec![0, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0],
            ]),
            11
        );

        assert_eq!(
            find_escape_route(vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 1],
                vec![0, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0],
            ]),
            11
        );

        assert_eq!(
            find_escape_route(vec![
                vec![0, 0, 0, 0, 1, 1, 1, 1],
                vec![1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 1, 0, 0, 0, 1, 1, 1],
                vec![1, 1, 0, 1, 0, 1, 1, 1],
                vec![1, 1, 0, 0, 0, 1, 1, 1],
                vec![1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 1, 1, 0, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 0, 0, 0, 0],
            ]),
            17
        );

        assert_eq!(
            find_escape_route(vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]),
            17
        );
    }
}

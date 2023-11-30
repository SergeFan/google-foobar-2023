use std::cmp::min;

const CHESS_BOARD_SIZE: isize = 8;

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn from(position: &Position) -> Position {
        Position { ..*position }
    }
}

type Matrix = [[isize; CHESS_BOARD_SIZE as usize]; CHESS_BOARD_SIZE as usize];

pub fn find_min_steps_to_target(src: isize, dest: isize) -> isize {
    // matrix to record minimum steps to reach the target by Knight
    // index of matrix represents absolute differences between (x, y) coordinates of Knight and the target
    // e.g. matrix[del_x][del_y] where del_x = abs(target.x - knight.x), del_y = abs(target.y - knight.y)
    let mut step_counts_matrix: Matrix =
        [[0; CHESS_BOARD_SIZE as usize]; CHESS_BOARD_SIZE as usize];

    let knight = Position {
        x: src % CHESS_BOARD_SIZE,
        y: src / CHESS_BOARD_SIZE,
    };

    let target = Position {
        x: dest % CHESS_BOARD_SIZE,
        y: dest / CHESS_BOARD_SIZE,
    };

    // special cases when Knight and the target locate at 4 corners of the chess board
    // with (0, 0) as left top corner coordinates
    if (knight.x == 0 && knight.y == 0 && target.x == 1 && target.y == 1)
        || (knight.x == 1 && knight.y == 1 && target.x == 0 && target.y == 0)
        // left top corner
        || (knight.x == CHESS_BOARD_SIZE - 1 && knight.y == 0 && target.x == CHESS_BOARD_SIZE - 2 && target.y == 1)
        || (knight.x == CHESS_BOARD_SIZE - 2 && knight.y == 1 && target.x == CHESS_BOARD_SIZE - 1 && target.y == 0)
        // right top corner
        || (knight.x == 0 && knight.y == CHESS_BOARD_SIZE - 1 && target.x == 1 && target.y == CHESS_BOARD_SIZE - 2)
        || (knight.x == 1 && knight.y == CHESS_BOARD_SIZE - 2 && target.x == 0 && target.y == CHESS_BOARD_SIZE - 1)
        // left bottom corner
        || (knight.x == CHESS_BOARD_SIZE - 1 && knight.y == CHESS_BOARD_SIZE - 1 && target.x == CHESS_BOARD_SIZE - 2 && target.y == CHESS_BOARD_SIZE - 2)
        || (knight.x == CHESS_BOARD_SIZE - 2 && knight.y == CHESS_BOARD_SIZE - 2 && target.x == CHESS_BOARD_SIZE - 1 && target.y == CHESS_BOARD_SIZE - 1)
    // right bottom corner
    {
        4 // steps to reach the target will always be 4
    } else {
        // special cases when Knight and the target locate within 2 x 3 squares area
        step_counts_matrix[1][0] = 3;
        step_counts_matrix[0][1] = 3;
        step_counts_matrix[1][1] = 2;
        step_counts_matrix[2][0] = 2;
        step_counts_matrix[0][2] = 2;
        step_counts_matrix[2][1] = 1;
        step_counts_matrix[1][2] = 1;

        count_steps(&mut step_counts_matrix, knight, &target)
    }
}

fn count_steps(step_counts_matrix: &mut Matrix, knight: Position, target: &Position) -> isize {
    if knight.x == target.x && knight.y == target.y {
        // Knight already stepped on the target, return 0
        step_counts_matrix[0][0]
    } else if step_counts_matrix[target.x.abs_diff(knight.x)][target.y.abs_diff(knight.y)] != 0 {
        // if fewest steps count has already been recorded, return the count
        step_counts_matrix[target.x.abs_diff(knight.x)][target.y.abs_diff(knight.y)]
    } else {
        // Knight can move towards 8 directions, but only 2 can get close to the target
        let mut potential_step_1 = Position::from(&knight);
        let mut potential_step_2 = Position::from(&knight);

        // possible steps Knight can take based on relative positions of Knight and the target
        if knight.x <= target.x && knight.y <= target.y {
            potential_step_1.x = knight.x + 2;
            potential_step_1.y = knight.y + 1;
            potential_step_2.x = knight.x + 1;
            potential_step_1.y = knight.y + 2;
        } else if knight.x <= target.x && knight.y > target.y {
            potential_step_1.x = knight.x + 2;
            potential_step_1.y = knight.y - 1;
            potential_step_2.x = knight.x + 1;
            potential_step_2.y = knight.y - 2;
        } else if knight.x > target.x && knight.y <= target.y {
            potential_step_1.x = knight.x - 2;
            potential_step_1.y = knight.y + 1;
            potential_step_2.x = knight.x - 1;
            potential_step_2.y = knight.y + 2;
        } else if knight.x > target.x && knight.y > target.y {
            potential_step_1.x = knight.x - 2;
            potential_step_1.y = knight.y - 1;
            potential_step_2.x = knight.x - 1;
            potential_step_2.y = knight.y - 2;
        }

        // record the fewer steps to reach the target by trying both possible steps
        step_counts_matrix[target.x.abs_diff(knight.x)][target.y.abs_diff(knight.y)] = min(
            count_steps(step_counts_matrix, potential_step_1, target),
            count_steps(step_counts_matrix, potential_step_2, target),
        ) + 1;

        // (x, y) or (y, x) are actually the same case
        step_counts_matrix[target.y.abs_diff(knight.y)][target.x.abs_diff(knight.x)] =
            step_counts_matrix[target.x.abs_diff(knight.x)][target.y.abs_diff(knight.y)];

        step_counts_matrix[target.x.abs_diff(knight.x)][target.y.abs_diff(knight.y)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_fewest_steps_to_target() {
        assert_eq!(find_min_steps_to_target(0, 1), 3);
        assert_eq!(find_min_steps_to_target(19, 36), 1);

        // 4 corners
        assert_eq!(find_min_steps_to_target(0, 9), 4);
        assert_eq!(find_min_steps_to_target(7, 14), 4);
        assert_eq!(find_min_steps_to_target(49, 56), 4);
        assert_eq!(find_min_steps_to_target(54, 63), 4);

        // random 2 squares
        assert_eq!(find_min_steps_to_target(56, 15), 5);
    }
}

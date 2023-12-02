pub fn count_staircase_variants(bricks: usize) -> usize {
    let mut variants: usize = 0;

    for first_stair_height in 1..=(bricks / 2) {
        variants += build_staircase(first_stair_height, bricks - first_stair_height);
    }

    variants
}

fn build_staircase(current_height: usize, left_bricks: usize) -> usize {
    if left_bricks < current_height + 1 {
        0
    } else if left_bricks < current_height * 2 + 3 {
        1
    } else {
        let mut variants: usize = 1;

        for gap in 1..=(left_bricks / 2 - current_height) {
            variants += build_staircase(current_height + gap, left_bricks - current_height - gap);
        }

        variants
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_staircase_variants() {
        assert_eq!(count_staircase_variants(3), 1);
        assert_eq!(count_staircase_variants(5), 2);

        // TODO: taking too much time (4s), slower than Java
        assert_eq!(count_staircase_variants(200), 487067745);
    }

    #[test]
    fn test_build_staircase() {
        assert_eq!(build_staircase(1, 4), 1);
        assert_eq!(build_staircase(2, 3), 1);
    }
}

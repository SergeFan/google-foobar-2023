use itertools::{enumerate, Itertools};

fn create_path(bunnies_to_rescue: &[usize], exit: usize) -> Vec<(usize, usize)> {
    let mut nodes_to_traverse = Vec::from(bunnies_to_rescue);

    // Insert entrance and exit
    nodes_to_traverse.insert(0, 0);
    nodes_to_traverse.push(exit);

    let mut path = Vec::new();

    for i in 1..nodes_to_traverse.len() {
        path.push((nodes_to_traverse[i - 1], nodes_to_traverse[i]));
    }

    path
}

pub fn rescue_bunnies(times: &mut Vec<Vec<isize>>, time_limit: isize) -> Vec<usize> {
    let row_numbers = times.len();
    let total_bunnies = row_numbers - 2;

    // Since interacting with security checkpoints (e.g. (i, k)) may add more time to the time limit,
    // then recalculate the time taken to (i, j) by trying to interact with checkpoints (i, k) and using alternate path (k, j)
    for k in 0..row_numbers {
        for i in 0..row_numbers {
            for j in 0..row_numbers {
                if times[i][j] > times[i][k] + times[k][j] {
                    times[i][j] = times[i][k] + times[k][j];
                }
            }
        }
    }

    // If rescuing a specific bunny takes time less than 0,
    // then all bunnies can be rescued by continuously looping that path
    for (i, time) in enumerate(times.iter()) {
        if time[i] < 0 {
            return (0..total_bunnies).collect();
        }
    }

    // Try to rescue as more bunnies as possible
    for i in (0..total_bunnies + 1).rev() {
        // Try different rescue order
        for bunnies_to_rescue in (1..total_bunnies + 1).permutations(i) {
            let mut total_time = 0;

            let path = create_path(&bunnies_to_rescue, row_numbers - 1);

            for (start, end) in path {
                total_time += times[start][end];
            }

            if total_time <= time_limit {
                return bunnies_to_rescue
                    .into_iter()
                    .map(|i| i - 1)
                    .sorted()
                    .collect();
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            rescue_bunnies(
                &mut vec![
                    vec![0, 1, 1, 1, 1],
                    vec![1, 0, 1, 1, 1],
                    vec![1, 1, 0, 1, 1],
                    vec![1, 1, 1, 0, 1],
                    vec![1, 1, 1, 1, 0],
                ],
                3,
            ),
            vec![0, 1]
        );

        assert_eq!(
            rescue_bunnies(
                &mut vec![
                    vec![0, 2, 2, 2, -1],
                    vec![9, 0, 2, 2, -1],
                    vec![9, 3, 0, 2, -1],
                    vec![9, 3, 2, 0, -1],
                    vec![9, 3, 2, 2, 0],
                ],
                1,
            ),
            vec![1, 2]
        );
    }
}

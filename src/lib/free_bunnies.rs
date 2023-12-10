use itertools::{enumerate, Itertools};

pub fn generate_keys(total: usize, required: usize) -> Vec<Vec<usize>> {
    // if any M out of N bunnies can unlock the door and any (M - 1) out of N cannot,
    // then each key has to be issued to (N + 1 - M) bunnies.
    // Because by this way, there will always be (N - (N + 1 - M) = M - 1) bunnies don't hold a specific key,
    // therefore any (M - 1) out of N bunnies cannot unlock the door,
    // and they have to have 1 additional bunny that holds the very key they are missing.

    // generate the combinations that each key should be issued to which (total + 1 - required) out of total bunnies
    let bunny_combinations_to_issue_key: Vec<Vec<usize>> =
        (0..total).combinations(total + 1 - required).collect();

    // the keys that each bunny are holding
    let mut key_list: Vec<Vec<usize>> = vec![vec![]; total];

    for (i, keys) in enumerate(&mut key_list) {
        // i: the index of each bunny
        for (key, bunny_combination_to_issue_key) in enumerate(&bunny_combinations_to_issue_key) {
            // for each key, if the combination of bunnies contains that No. i bunny, then issue the key
            if bunny_combination_to_issue_key.contains(&i) {
                keys.push(key)
            }
        }
    }

    key_list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_keys() {
        assert_eq!(generate_keys(2, 1), vec![vec![0], vec![0]]);

        assert_eq!(
            generate_keys(4, 4),
            vec![vec![0], vec![1], vec![2], vec![3]]
        );

        assert_eq!(
            generate_keys(5, 3),
            vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![0, 1, 2, 6, 7, 8],
                vec![0, 3, 4, 6, 7, 9],
                vec![1, 3, 5, 6, 8, 9],
                vec![2, 4, 5, 7, 8, 9],
            ]
        );
    }
}

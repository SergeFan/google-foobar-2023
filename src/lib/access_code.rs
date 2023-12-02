use std::cmp::Ordering;

pub fn find_lucky_triples(code_list: Vec<usize>) -> usize {
    let mut lucky_triples_variants: usize = 0;

    match code_list.len().cmp(&3) {
        Ordering::Less => return 0,
        Ordering::Equal => {
            if code_list[2] % code_list[1] == 0 && code_list[1] % code_list[0] == 0 {
                return 1;
            }
        }
        Ordering::Greater => {
            let list_len = code_list.len();

            for i in 0..list_len - 2 {
                for j in i + 1..list_len - 1 {
                    if code_list[j] % code_list[i] != 0 {
                        continue;
                    }
                    for k in j + 1..list_len {
                        if code_list[k] % code_list[j] != 0 {
                            continue;
                        }
                        lucky_triples_variants += 1;
                    }
                }
            }
        }
    }

    lucky_triples_variants
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lucky_triples() {
        assert_eq!(find_lucky_triples(vec![1, 1, 1]), 1);
        assert_eq!(find_lucky_triples(vec![1, 2, 3, 4, 5, 6]), 3);
    }
}

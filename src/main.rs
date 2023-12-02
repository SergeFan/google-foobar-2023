use google_foobar_2023::lib::access_code::find_lucky_triples;
use google_foobar_2023::lib::braille::convert_str_to_braille_code;
use google_foobar_2023::lib::ion_flux::find_ion_flux_labels;
use google_foobar_2023::lib::not_volunteered::find_min_steps_to_target;
use google_foobar_2023::lib::staircase::count_staircase_variants;

fn main() {
    // Level 1
    // Challenge 1: Braille Translation
    assert_eq!(
        convert_str_to_braille_code("Braille"),
        "000001110000111010100000010100111000111000100010"
    );

    // Level 2
    // Challenge 1: Ion Flux Relabeling
    assert_eq!(find_ion_flux_labels(3, vec![7, 3, 5, 1]), vec![-1, 7, 6, 3]);

    // Challenge 2: Don't get volunteered!
    assert_eq!(find_min_steps_to_target(56, 15), 5);

    // Level 3
    // Challenge 1: The Grandest Staircase Of Them All
    assert_eq!(count_staircase_variants(200), 487067745);

    // Challenge 2: Find the Access Code
    assert_eq!(find_lucky_triples(vec![1, 1, 1]), 1);
    assert_eq!(find_lucky_triples(vec![1, 2, 3, 4, 5, 6]), 3);

    println!("All challenges completed.")
}

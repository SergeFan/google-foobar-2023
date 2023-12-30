use google_foobar_2023::lib::access_code::find_lucky_triples;
use google_foobar_2023::lib::braille::convert_str_to_braille_code;
use google_foobar_2023::lib::escape::find_escape_route;
use google_foobar_2023::lib::expanding_nebula::count_previous_status;
use google_foobar_2023::lib::free_bunnies::generate_keys;
use google_foobar_2023::lib::ion_flux::find_ion_flux_labels;
use google_foobar_2023::lib::not_volunteered::find_min_steps_to_target;
use google_foobar_2023::lib::running_with_bunnies::rescue_bunnies;
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
    assert_eq!(count_staircase_variants(5), 2);

    // Challenge 2: Find the Access Code
    assert_eq!(find_lucky_triples(vec![1, 1, 1]), 1);
    assert_eq!(find_lucky_triples(vec![1, 2, 3, 4, 5, 6]), 3);

    // Challenge 3: Prepare the Bunnies' Escape
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

    // Level 4
    // Challenge 1: Free the Bunny Workers
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

    // Challenge 2: Running with Bunnies
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

    // Level 5
    // Challenge 1: Expanding Nebula
    assert_eq!(
        count_previous_status(vec![
            vec![true, false, true, false, false, true, true, true],
            vec![true, false, true, false, false, false, true, false],
            vec![true, true, true, false, false, false, true, false],
            vec![true, false, true, false, false, false, true, false],
            vec![true, false, true, false, false, true, true, true],
        ]),
        254
    );

    // For your eyes only!
    println!("b'D0IHBhYQAB1dT0FUVEITARASEUkCSEYNGwkYFhQUEAsJSFtOUwAHBxAWCAtKT01OUwASFRoBER0J\nSFtOUwwaEAcWAQdMBARJWEVTEhYbDAtYDQwLGhFTU09TQhtABA4NHwAQVFlTQhxPCgMHABZTU09T\nQh1PDgRJWEVTFRocQk4USEYZHQtVVAg='");

    println!("All challenges completed.")
}

use google_foobar_2023::lib::braille::convert_str_to_braille_code;
use google_foobar_2023::lib::ion_flux::find_ion_flux_labels;

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
}

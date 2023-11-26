use google_foobar_2023::lib::braille::convert_str_to_braille_code;
use google_foobar_2023::lib::ion_flux::find_ion_flux_labels;

fn main() {
    // Challenge 1: Braille Translation
    println!(
        "Braille Translation: {}",
        convert_str_to_braille_code("Braille Translation")
    );

    // Challenge 2: Ion Flux Relabeling
    assert_eq!(find_ion_flux_labels(3, vec![7, 3, 5, 1]), vec![-1, 7, 6, 3]);
}

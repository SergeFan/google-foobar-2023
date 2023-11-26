use google_foobar_2023::lib::braille::convert_str_to_braille_code;

fn main() {
    // Level 1
    // Challenge 1: Braille Translation
    assert_eq!(
        convert_str_to_braille_code("Braille"),
        "000001110000111010100000010100111000111000100010"
    );
}

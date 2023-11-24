use phf::phf_map;

static BRAILLE_LETTERS: phf::Map<char, u8> = phf_map! {
    'a' => 0b100_000,
    'b' => 0b110_000,
    'c' => 0b100_100,
    'd' => 0b100_110,
    'e' => 0b100_010,
    'f' => 0b110_100,
    'g' => 0b110_110,
    'h' => 0b110_010,
    'i' => 0b010_100,
    'j' => 0b010_110,
};

const PADDLE_1: u8 = 0b001_000;
const PADDLE_2: u8 = 0b001_001;
const CAPITAL: u8 = 0b000_001;
const WHITE_SPACE: u8 = 0b000_000;

pub fn convert_str_to_braille_code(str: &str) -> String {
    let mut braille_code: String = "".to_string();

    for c in str.chars() {
        braille_code += convert_char_to_braille(c).as_str();
    }

    braille_code
}

fn convert_char_to_braille(c: char) -> String {
    match c {
        'a'..='j' => {
            format!(
                "{:06b}",
                BRAILLE_LETTERS
                    .get(&c)
                    .expect("letter out of range ('a' to 'j')")
            )
        }

        'k'..='t' => {
            format!(
                "{:06b}",
                BRAILLE_LETTERS
                    .get(&((c.to_ascii_lowercase() as u8 - 10) as char))
                    .expect("letter out of range ('k' to 't')")
                    + PADDLE_1
            )
        }

        'u'..='v' => {
            format!(
                "{:06b}",
                BRAILLE_LETTERS
                    .get(&((c.to_ascii_lowercase() as u8 - 20) as char))
                    .unwrap()
                    + PADDLE_2
            )
        }

        'w' => {
            format!("{:06b}", BRAILLE_LETTERS.get(&'j').unwrap() + CAPITAL)
        }

        'x'..='z' => {
            format!(
                "{:06b}",
                BRAILLE_LETTERS
                    .get(&((c.to_ascii_lowercase() as u8 - 21) as char))
                    .unwrap()
                    + PADDLE_2
            )
        }

        ' ' => {
            format!("{:06b}", WHITE_SPACE)
        }

        _ if c.is_uppercase() => {
            format!("{:06b}", CAPITAL) + convert_char_to_braille(c.to_ascii_lowercase()).as_str()
        }

        _ => "".to_string(),
    }
}

#[test]
fn test_convert_str_to_braille_code() {
    assert_eq!(
        convert_str_to_braille_code("code"),
        "100100101010100110100010"
    );
    assert_eq!(
        convert_str_to_braille_code("Braille"),
        "000001110000111010100000010100111000111000100010"
    );
    assert_eq!(
        convert_str_to_braille_code("Hello World"),
        "000001110010100010111000111000101010000000000001010111101010111010111000100110"
    );
    assert_eq!(
        convert_str_to_braille_code("The quick brown fox jumps over the lazy dog"),
        "0000010111101100101000100000001111101010010101001001001010000000001100001110101010100101111\
        011100000001101001010101011010000000101101010011011001111000111000000001010101110011000101110\
        10000000011110110010100010000000111000100000101011101111000000100110101010110110"
    );
}

#[test]
fn test_convert_char_to_braille() {
    assert_eq!(convert_char_to_braille('a'), "100000");
    assert_eq!(convert_char_to_braille('j'), "010110");
    assert_eq!(convert_char_to_braille('k'), "101000");
    assert_eq!(convert_char_to_braille('t'), "011110");
    assert_eq!(convert_char_to_braille('u'), "101001");
    assert_eq!(convert_char_to_braille('z'), "101011");
    assert_eq!(convert_char_to_braille('w'), "010111");
    assert_eq!(convert_char_to_braille('A'), "000001100000");
    assert_eq!(convert_char_to_braille(' '), "000000");
    assert_eq!(convert_char_to_braille('*'), "");
}

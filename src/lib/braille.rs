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

fn get_braille_code(c: char) -> u8 {
    match c {
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
        _ => 0b000_000,
    }
}

fn convert_char_to_braille(c: char) -> String {
    match c {
        'a'..='j' => {
            format!("{:06b}", get_braille_code(c))
        }

        'k'..='t' => {
            format!(
                "{:06b}",
                get_braille_code((c.to_ascii_lowercase() as u8 - 10) as char) + PADDLE_1
            )
        }

        'u'..='v' => {
            format!(
                "{:06b}",
                get_braille_code((c.to_ascii_lowercase() as u8 - 20) as char) + PADDLE_2
            )
        }

        'w' => {
            format!("{:06b}", get_braille_code('j') + CAPITAL)
        }

        'x'..='z' => {
            format!(
                "{:06b}",
                get_braille_code((c.to_ascii_lowercase() as u8 - 21) as char) + PADDLE_2
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

#[cfg(test)]
mod test_braille {
    use super::*;

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
            "00000101111011001010001000000011111010100101010010010010100000000011000011101\
            010101001011110111000000011010010101010110100000001011010100110110011110001110\
            000000010101011100110001011101000000001111011001010001000000011100010000010101\
            1101111000000100110101010110110"
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
}

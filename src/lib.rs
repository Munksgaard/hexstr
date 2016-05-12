#![feature(step_by)]

fn to_byte(hexchr: u8) -> u8 {
    match hexchr {
        n if ('a' as u8) <= hexchr && hexchr <= ('f' as u8) =>
            10 + n - ('a' as u8),
        n if ('A' as u8) <= hexchr && hexchr <= ('F' as u8) =>
            10 + n - ('A' as u8),
        n if ('0' as u8) <= hexchr && hexchr <= ('9' as u8) =>
            n - ('0' as u8),
        _ => panic!("Not a hex string!")
    }
}

pub fn parse(hexstr: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let start =
        if hexstr.len() % 2 != 0 {
            bytes.push(to_byte(hexstr[0]));
            1
        } else {
            0
        };

    for i in (start..hexstr.len()).step_by(2) {
        bytes.push(to_byte(hexstr[i]) * 16 + to_byte(hexstr[i+1]));
    }

    bytes
}


pub fn print_bytes(bytes: &[u8]) {
    for chunk in bytes.chunks(16) {
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cryptopals1_parse() {
        let bytes = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected: &[u8] = b"I'm killing your brain like a poisonous mushroom";

        let actual = parse(bytes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse() {
        let bytes = b"7041ab";

        let expected = &[0x70, 0x41, 0xab];

        let actual = parse(bytes);

        assert_eq!(actual, expected);
    }
}

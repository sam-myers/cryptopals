#[cfg(test)]
mod tests {
    use crate::ciphers_from_file;
    use crate::xor::*;

    use serialize::base64::{self, ToBase64};
    use serialize::hex::FromHex;

    #[test]
    fn test_challenge1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let result = input.from_hex().unwrap().to_base64(base64::STANDARD);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_challenge2() {
        let test = "1c0111001f010100061a024b53535009181c";
        let key = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let result = xor(&test.from_hex().unwrap(), &key.from_hex().unwrap());

        assert_eq!(result, expected.from_hex().unwrap());
    }

    #[test]
    fn test_challenge3() {
        let cipher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            .from_hex()
            .unwrap();
        let search = XorSearch::new(cipher_text);
        match search.search_single_bit() {
            XorSearchResult::Found(s, k) => {
                assert_eq!(s, "Cooking MC's like a pound of bacon".to_string());
                assert_eq!(k.key, vec!['X' as u8]);
            }
            XorSearchResult::NotFound => assert!(false),
        }
    }

    #[test]
    fn test_challenge4() {
        for cipher in ciphers_from_file("resources/set1_challenge4.txt").unwrap() {
            match XorSearch::new(cipher).search_single_bit() {
                XorSearchResult::Found(s, k) => {
                    assert_eq!(s, "Now that the party is jumping\n".to_string());
                    assert_eq!(k.key, vec!['5' as u8]);
                }
                _ => {}
            }
        }
    }

//    #[test]
//    fn test_challenge5() {
//        let clear_text = "Burning 'em, if you ain't quick and nimble
//I go crazy when I hear a cymbal";
//        let cipher_text = xor(clear_text.as_bytes(), &"ICE".as_bytes());
//
//        assert_eq!(cipher_text.to_base64(base64::STANDARD),
//                   "CzY3JyorLmNiLC5paSojaToqPGMkIC1iPWM0PComImMkJydlJyooKy8gQwplLixlKjEkMzplPisgJ2MMaSsgKDFlKGMmMC4nKC8=".to_string())
//    }
}

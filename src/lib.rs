extern crate regex;

use regex::Regex;

const LOWERCASE_CHARS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const UPPERCASE_CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

/// Encrypts a text with given shift using Caesar
/// 
/// # Examples
/// 
/// ```
/// encrypt("abcd", &1u8)
/// ```
pub fn encrypt<'a>(secret: &'a str, shift: &'a u8) -> Result<String, &'a str> {
    match validations(secret, shift) {
        None => (),
        Some(v) => return Err(v),
    };

    let mut result: String = String::new();

    for c in secret.chars() {
        if c.is_whitespace() {
            result.push(c.clone());
        } else if c.is_uppercase() {
             result.push(
                 shift_right_char(&c, UPPERCASE_CHARS, &shift).clone()
             )
        } else {
            result.push(
                shift_right_char(&c, LOWERCASE_CHARS, &shift)
            )
        }
    }

    Ok(result)
}

/// Decrypts a text with given shift using Caesar
/// 
/// # Examples
/// 
/// ```
/// decrypt("abcd", &1u8)
/// ```
pub fn decrypt<'a>(secret: &'a str, shift: &'a u8) -> Result<String, &'a str> {
    match validations(&secret, &shift) {
        None => (),
        Some(v) => return Err(v),
    };

    let mut result: String = String::new();

    for c in secret.chars() {
        if c.is_whitespace() {
            result.push(c.clone());
        } else if c.is_uppercase() {
             result.push(
                 shift_left_char(&c, UPPERCASE_CHARS, &shift)
             )
        } else {
            result.push(
                shift_left_char(&c, LOWERCASE_CHARS, &shift)
            )
        }
    }

    Ok(result)
}

fn validations<'a>(secret: &str, shift: &u8) -> Option<&'a str> {
    if *shift > 25u8 {
        return Some("shift can not be > 25")
    }

    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
    if !re.is_match(secret) {
        return Some("secret has to match regex [a-zA-Z]+")
    }

    None
}

fn shift_right_char(c: &char, char_vec: [char; 26], shift: &u8) -> char {
    let current_index: u8 = char_vec.iter().position(|&x| x == *c).unwrap() as u8;

    let mut new_index: u8 = current_index + shift;

    if new_index > 25u8 {
        new_index -= 26;
    }

    return char_vec[new_index as usize]
}

fn shift_left_char(c: &char, char_vec: [char; 26], shift: &u8) -> char {
    let current_index: u8 = char_vec.iter().position(|&x| x == *c).unwrap() as u8;

    let mut new_index: i16 = current_index as i16 - *shift as i16;

    if new_index < 0i16 {
        new_index = 26 + new_index;
    }

    return char_vec[new_index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("abyz", &25u8).unwrap(), "zaxy");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("abyz", &25u8).unwrap(), "bcza");
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let alphabet: &str = "abcdefghijklmnopqrstuvwxy";

        let mut rng = rand::thread_rng();
        let random_key: u8 = rng.gen_range(0, 26);

        let encrypted: String = encrypt(alphabet, &random_key).unwrap();
        let decrypted: String = decrypt(&encrypted, &random_key).unwrap();

        assert_eq!(alphabet, decrypted);
    }
}
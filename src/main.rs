use std::{collections::HashMap, io, path::Path};

#[allow(dead_code)]
fn read_file_content(input: &Path) -> io::Result<String> {
    if !input.is_file() {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    let content = std::fs::read_to_string(input)?;
    Ok(content)
}

#[allow(dead_code)]
fn get_letter_frequency_count(data: String) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for character in data.chars() {
        *frequencies.entry(character).or_insert(1) += 1;
    }

    frequencies
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    pub fn test_read_file() {
        let input_file = Path::new("test_data.txt");
        let content = read_file_content(input_file).unwrap();

        assert!(!content.is_empty());
        assert!(content.contains("Victor Hugo"));
        assert!(content.contains("Title: Les Misérables"));
    }

    #[test]
    pub fn test_read_file_invalid_input() {
        let input_file = Path::new("test_invalidtxt");
        let content = read_file_content(input_file);
        assert!(content.is_err());
    }

    #[test]
    pub fn test_frequncies_in_test_data() {
        let input_file = Path::new("test_data.txt");
        let content = read_file_content(input_file).unwrap();

        let frequencies = get_letter_frequency_count(content);
        let fqy_capital_x = frequencies[&'X'];
        let fqy_t = frequencies[&'t'];

        assert_eq!(fqy_capital_x, 334);
        assert_eq!(fqy_t, 223001);
    }
}

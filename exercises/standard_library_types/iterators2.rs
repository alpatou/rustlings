// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.
pub fn capitalize_first(input: &str) -> String {
    let mut first_letter_capitalized: bool = false;
    let mut result: String = String::new();

    let mut c = input.chars();

    while let Some(c) = c.next() {
        if !first_letter_capitalized {
            result += &c.to_uppercase().to_string();
            first_letter_capitalized = true;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut capitalized_result = Vec::new();

    for word in words {
        let mut capitalized_word = capitalize_first(word);
        capitalized_result.push(capitalized_word);
    }
    capitalized_result
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut final_sentence: String = String::new();
    for word in words {
        final_sentence += capitalize_first(&word).as_str();
    }
    final_sentence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

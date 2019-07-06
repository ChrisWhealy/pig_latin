fn main() {
  let plain_text = String::from("This is a Rust training exercise on how to transform the words in an English UTF-8 string into pig latin");
  let mut result = String::new();

  for word in plain_text.split_whitespace() {
    result = format!("{} {}", result, pigify(&word));
  }

  println!("\"{}\" in pig latin is \"{}\"", plain_text, result.trim_start());
}

fn pigify(word: &str) -> String {
  // Since no English word starts with a consonent cluster longer than 3 characters,
  // we can take a brute force approach here
  let (first_char, rem1) = split_first_character(word);

  return
    if is_roman_vowel(first_char) {
      format!("{}-hay", word)
    }
    else {
      let (second_char, rem2) = split_first_character(rem1);

      if is_roman_vowel(second_char) {
        format!("{}-{}ay", rem1, first_char)
      }
      else {
        let (third_char, rem3) = split_first_character(rem2);

        if is_roman_vowel(third_char) {
          format!("{}-{}{}ay", rem2, first_char, second_char)
        }
        else {
          format!("{}-{}{}{}ay", rem3, first_char, second_char, third_char)
        }
      }
    }
}

fn is_roman_vowel(c: &str) -> bool {
  ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"].contains(&c)
}

fn split_first_character(word: &str) -> (&str, &str) {
  for i in 1..5 {
    match word.get(0..i) {
      Some(unicode_char) => return (unicode_char, &word[i..])
    , None               => ()
    }
  }

  (&word[0..0], word)
}

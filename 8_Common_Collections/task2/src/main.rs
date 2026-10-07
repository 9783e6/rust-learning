
fn convert_to_pig_latin(string: &str) -> String {
    let mut output = String::new();
    for word in string.split_whitespace() {
        let mut out_word = String::new();
        let first_char = word.chars().next().unwrap();
        if ['q', 'w', 'r', 't', 'p', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm'].contains(&first_char) {
            out_word.push_str(&word[first_char.len_utf8()..]);
            out_word.push('-');
            out_word.push_str(&word[..1]);
            out_word.push_str("ay");
        } else {
            out_word.push_str(word);
            out_word.push_str("-hay");
        }

        output.push_str(&out_word);
        output.push(' ');
    }
    output
}

fn main() {
    let input = "hello world";
    println!("{}", input);
    println!("{}", convert_to_pig_latin(input));
}

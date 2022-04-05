//
// Handling Plurality
//

fn print_words(sentence: &str) {
    let words: Vec<_> = sentence.split_whitespace().collect();
    match words.as_slice() {
        [] => println!("There were no words"),
        [word] => println!("Found one word: {}", word),
        _ => println!("Found {} words: {:?}", words.len(), words),
    }
}

fn test_handling_plurality() {
    print_words("");
    print_words("Hello");
    print_words("Hello world!");
}

//
// Matching the start of a slice
//

fn is_elf(sentence: &[char]) {
    match sentence {
        ['e', 'l', 'f', ..] => println!("We have an elf"),
        _ => println!("No elf here"),
    }
}

fn test_start_slice() {
    is_elf(&['e', 'l', 'f', 'f', 'y']);
    is_elf(&['a']);
}

//
// Checking for palindromes
//

fn is_palindrome(items: &[char]) -> bool {
    match items {
        [first, middle @ .., last] => first == last && is_palindrome(middle),
        [] | [_] => true,
    }
}

fn test_palindrome() {
    println!("palindrome {}", is_palindrome(&['a', 'b', 'a']));
    println!("palindrome {}", is_palindrome(&['a', 'b', 'b']));
}

//
// Irrefutable pattern matching
//

fn format_coordinates([x, y]: [f32; 2]) -> String {
    format!("{}|{}", x, y)
}

fn test_format_coordinates() {
    let point = [3.14, -42.0];

    println!("{}", format_coordinates(point));

    let [x, y] = point;
    println!("x: {}, y: {}", x, y);
}

fn main() {
    test_handling_plurality();
    test_start_slice();
    test_palindrome();
    test_format_coordinates();
}

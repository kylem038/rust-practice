use std::vec;

fn main() {
    let days: Vec<String> = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
        String::from("fifth"),
        String::from("sixth"),
        String::from("seventh"),
        String::from("eigth"),
        String::from("nineth"),
        String::from("tenth"),
        String::from("eleventh"),
        String::from("twelfth"),
    ];

    let lyrics: Vec<String> = vec![
        String::from("A partridge in a pear tree"),
        String::from("Two turtle doves"),
        String::from("Three french hens"),
        String::from("Four calling birds"),
        String::from("Five golden rings"),
        String::from("Six geese a-laying"),
        String::from("Seven swans a-swimming"),
        String::from("Eight maids a-milking"),
        String::from("Nine ladies dancing"),
        String::from("Ten lords a-leaping"),
        String::from("Eleven pipers piping"),
        String::from("Twelve drummers drumming"),
    ];

    let mut counter: usize = 0;
    let mut lyric_index: usize = 1;
    let n: usize = 11;

    loop {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[counter]
        );

        while lyric_index > 0 {
            println!("{}", lyrics[lyric_index - 1]);
            lyric_index -= 1;
        }

        if counter == n {
            break;
        } else {
            counter += 1;
            lyric_index = 1 + counter;
        }
    }

    // On the first day of Christmas my true love sent to me
    // A partridge in a pear tree

    // On the second day of Christmas my true love sent to me
    // Two turtle doves, and
    // A partridge in a pear tree
}

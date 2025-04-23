const START:[&'static str; 2] = [
    "On the",
    "day of Christmas\n\
    My true love sent to me"
];

const MIDDLE:[&'static str; 11] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings (five golden rings)",
    "Four calling birds",
    "Three French hens",
    "Two turtle-doves"
];

const END:[&'static str; 3] = [
    "A",
    "And a",
    "partridge in a pear tree",
];

const NUMERALS:[&'static str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

fn main() {
    println!("\n{} {} {}", START[0], NUMERALS[0], START[1]);
    println!("{} {}", END[0], END[2]);

    for i in 1..12 {

        println!("\n{} {} {}", START[0], NUMERALS[i], START[1]);

        for j in &MIDDLE[(11-i)..] {
            println!("{}", j);
        }

        println!("{} {}", END[1], END[2]);

    }

    println!("{} {}", END[1], END[2]);
}
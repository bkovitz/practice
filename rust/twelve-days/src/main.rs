fn main() {
    let ordinals = [
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

    let gifts = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 0..=11 {
        println!("On the {} day of Christmas, my true love sent to me", ordinals[day]);
        for i in (0..=day).rev() {
            if i == 0 {
                println!("{} {}.", if day == 0 { "A" } else {"And a"}, gifts[i]);
            } else {
                println!("{},", gifts[i]);
            }
        }
        if day != 11 {
            println!();
        }
    }
}
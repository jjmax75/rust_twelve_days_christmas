fn main() {
    let ordinals = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth",
        "Seventh", "Eigth", "Ninth", "Tenth", "Eleventh", "Twelfth"];

    let song_lines = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming",
        "Eight maids-a-milking",
        "Nine ladies dancing",
        "Ten lords-a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for (mut index, ordinal) in ordinals.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", ordinal);

        if index == 0 {
            println!("{}", song_lines[0]);
            continue;
        }

        while index > 0 {
            println!("{}", song_lines[index]);

            index -= 1;
        }

        println!("And {}", song_lines[0]);

        println!("");

    }
}

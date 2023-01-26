fn main() {
    // another_function();
    let texts = [
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "I sent 11 pipers piping",
        "12 drummers drumming",
    ];
    // let mut index: u32 = 0;
    let mut finish = String::from("On the first day of Christmas\t\nMy true love sent to me\n");
    let finish1 = String::from("A partridge in a pear tree");
    for x in texts {
        println!("{} \n{}", finish, finish1);
        finish += "\t\n";
        finish += x;
    }
}

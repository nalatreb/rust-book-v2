fn main() {
    let days = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for n in 0..12 {
        let day = n + 1;
        println!("On the {day} day of Christmas my true love sent to me");

        for t in (0..day).rev() {
            let what = days[t];
            println!("{what}");
        }
    }
}

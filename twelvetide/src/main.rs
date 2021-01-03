const VERSES: i32 = 12;

fn main() {

    for i in 1..VERSES+1 {
        print!("On the ");
        twelvetide_intro(i);
        println!(" day of Christmas my true love sent to me");
        twelvetide_rest(i);
        println!();
    }
}

fn twelvetide_intro(rep: i32) {
    match rep {
        1  => print!("first"),
        2  => print!("second"),
        3  => print!("third"),
        4  => print!("fourth"),
        5  => print!("fifth"),
        6  => print!("sixth"),
        7  => print!("seventh"),
        8  => print!("eighth"),
        9  => print!("ninth"),
        10 => print!("tenth"),
        11 => print!("eleventh"),
        12 => print!("twelveth"),
        _  => (),
    }
}

fn twelvetide_rest(mut rep: i32) {

    if rep == 1 {
        println!("A partridge in a pear tree");
    }
    else {
        while rep > 0 {
            match rep {
                12 => println!("Twelve drummers drumming"),
                11 => println!("Eleven pipers piping"),
                10 => println!("Ten lords a-leaping"),
                9  => println!("Nine ladies dancing"),
                8  => println!("Eight maids a-milking"),
                7  => println!("Seven swans a-swimming"),
                6  => println!("Six geese a-laying"),
                5  => println!("Five gold rings"),
                4  => println!("Four calling birds"),
                3  => println!("Three French hens"),
                2  => println!("Two turtle doves"),
                1  => println!("And a partridge in a pear tree"),
                _  => (),
            }
            rep -= 1;
        }
    }
}

use std::io;
use String;

fn main() {

    println!("Input \"exit\" to break the loop!");

    loop {

        let mut f_letter = FirstLetter {
            letter: ' ',
            read: false,
            vowel: false,
        };

        let mut string = String::new();
        io::stdin()
            .read_line(&mut string)
            .expect("Error while reading stdin");

        let string = string.trim();

        if string.trim() == String::from("exit") { break }

        for c in string.trim().chars() {
            if f_letter.read == false {

                f_letter.read = true;
                f_letter.letter = c;
                f_letter.vowel_check();

                if f_letter.vowel == true {
                    print!("{}", f_letter.letter);
                }
                continue;
            }

            print!("{}", c);
        }

        if f_letter.vowel == true {
            println!("-hay");
        }
        else {
            println!("-{}ay", f_letter.letter);
        }
    }
}

struct FirstLetter {
    vowel: bool,
    letter: char,
    read: bool,
}

impl FirstLetter {
    fn vowel_check(&mut self) {
        let c = self.letter;
        match c {
            'a' => self.vowel = true,
            'i' => self.vowel = true,
            'u' => self.vowel = true,
            'e' => self.vowel = true,
            'o' => self.vowel = true,
            'y' => self.vowel = true,
            _ => self.vowel = false,
        }
    }
}

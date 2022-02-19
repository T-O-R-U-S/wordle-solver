mod dict;

use dict::Contains::*;

use std::io::stdin;

fn main() {
    let mut dict: Option<Vec<&str>> = None;
    loop {
        let mut criteria = vec![];
        let mut input = String::with_capacity(10);

        stdin().read_line(&mut input).unwrap();

        if &input == "exit\n" {
            std::process::exit(0)
        }

        if &input == "reset\n" {
            println!("Resetting..!\n");
            dict = None;
            continue
        }

        let mut input = input.trim().chars().enumerate();

        while let Option::Some((_, op)) = input.next() {
            match op {
                '+' => {
                    let (chr_pos, chr) = input.next().unwrap();
                    criteria.push(Exact(chr, (chr_pos - 1) / 2));
                },
                '*' => {
                    let (chr_pos, chr) = input.next().unwrap();
                    criteria.push(NoneAt(chr, (chr_pos - 1) / 2));
                }
                '!' => {
                    let (_, chr) = input.next().unwrap();
                    criteria.push(No(chr));
                }
                '^' => {
                    let (chr_pos, chr) = input.next().unwrap();
                    criteria.push(Some(chr, (chr_pos - 1) / 2));
                },
                inp => panic!("didn't expect {inp}"),
            }
        }

        dict = Option::Some(dict::dict(&criteria, dict));

        println!(
            "List of suggestions:\n\n{}\n\nGuesses: {} from {}\n\n",
            dict.as_ref().unwrap().join("\n"),
            dict.as_ref().unwrap().len(),
            dict::initial().len()
        );

        println!("Top pick: {:?}", dict.as_ref().unwrap()[dict.as_ref().unwrap().len()-1]);
    }
}

mod dict;

use dict::Contains::*;

use std::io::stdin;
use std::io::Read;

fn main() {
    let mut dict: Option<Vec<&str>> = None;
    loop {
        let mut criteria = vec![];
        let mut input = String::with_capacity(10);

        stdin().read_line(&mut input).unwrap();

        if &input == "exit\n" {
            std::process::exit(0)
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
                    let (chr_pos, chr) = input.next().unwrap();
                    criteria.push(No(chr));
                }
                '^' => {
                    let (chr_pos, chr) = input.next().unwrap();
                    criteria.push(Some(chr));
                },
                chr @ 'a'..='z' => criteria.push(Some(chr)),
                inp => panic!("didn't expect {inp}"),
            }
        }

        dict = Option::Some(dict::dict(&criteria, dict));

        println!(
            "{dict:#?}\n\nGuesses: {} from {}",
            dict.as_ref().unwrap().len(),
            dict::initial().len()
        );
    }
}

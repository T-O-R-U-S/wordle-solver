mod dict;

use dict::Contains::*;

use std::io::stdin;
use std::io::Read;

fn main() {
    let mut criteria = vec![];
    loop {
        let mut input = String::with_capacity(10);

        stdin().read_line(&mut input).unwrap();

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

        let dict = dict::dict(&criteria);

        println!(
            "{dict:#?}\n\nGuesses: {} from {}",
            dict.len(),
            dict::initial().len()
        );
    }
}

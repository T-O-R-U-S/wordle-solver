use std::collections::HashSet;

/// Tells the guesser that it contains
/// zero of XYZ letter, some letter in
/// an unknown position, and some letter
/// in an exact position
#[derive(Debug)]
pub enum Contains {
	No(char),
	NoneAt(char, usize),
	Some(char, usize),
	Exact(char, usize),
}

pub fn initial() -> Vec<&'static str> {
	include_str!("../words.txt").split("\n").collect::<Vec<&str>>()
}

pub fn dict<'a>(contains: &[Contains], words: Option<Vec<&'a str>>) -> Vec<&'a str> {
	// We know that all the words in words.txt are
	// already 5-characters long, so we don't
	// need to filter them.
	let words = match words {
		None => include_str!("../words.txt").split("\n").collect::<Vec<&str>>().into_iter(),
		Some(words) => words.into_iter()
	};

	let mut words = words.filter(|x| {
		for cond in contains {
			match cond {
				Contains::No(chr) if x.chars().any(|v| &v == chr) => return false,
				Contains::Some(chr, pos) if !x.chars().any(|v| &v == chr) || x.chars().nth(*pos).unwrap() == *chr => return false,
				Contains::Exact(chr, pos) if x.chars().nth(*pos).unwrap() != *chr => return false,
				Contains::NoneAt(chr, pos) if x.chars().nth(*pos).unwrap() == *chr => return false,
				_ => {}
			}
		}
		true
	 }).collect::<Vec<&str>>();

	words.sort_by(|a, b| {
		let mut a_set = HashSet::new();
		let mut b_set = HashSet::new();

		let mut a_weight = 0;
		let mut b_weight = 0;

		for x in a.chars() {
			a_set.insert(x);
		}
		for x in b.chars() {
			b_set.insert(x);
		}

		a_weight += a_set.len();
		b_weight += b_set.len();

		for letter in a_set {
			a_weight += letter_weight(letter);
		}

		for letter in b_set {
			b_weight += letter_weight(letter);
		}

		a_weight.partial_cmp(&b_weight).unwrap()
	});

	words
}

fn letter_weight(letter: char) -> usize {
	match letter {
		'e' => 25,
		'a' => 24,
		'r' => 23,
		'i' => 22,
		'o' => 21,
		't' => 20,
		'n' => 19,
		's' => 18,
		'l' => 17,
		'c' => 16,
		'u' => 15,
		'd' => 14,
		'p' => 13,
		'm' => 12,
		'h' => 11,
		'g' => 10,
		'b' => 9,
		'f' => 8,
		'y' => 7,
		'w' => 6,
		'k' => 5,
		'v' => 4,
		'x' => 3,
		'z' => 2,
		'j' => 1,
		'q' => 0,
		a if !a.is_alphabetic() => panic!("Not alphabetical"),
		a => panic!("Unimplemented letter {a}")
	}
}
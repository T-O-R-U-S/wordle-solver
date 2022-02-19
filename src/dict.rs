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

	words.filter(|x| {
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
	 }).collect::<Vec<&str>>()
}
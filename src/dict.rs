/// Tells the guesser that it contains
/// zero of XYZ letter, some letter in
/// an unknown position, and some letter
/// in an exact position
#[derive(Debug)]
pub enum Contains {
	No(char),
	NoneAt(char, usize),
	Some(char),
	Exact(char, usize),
}

pub fn initial() -> Vec<&'static str> {
	include_str!("../words.txt").split("\n").collect::<Vec<&str>>()
}

pub fn dict(contains: &[Contains]) -> Vec<&str> {
	// We know that all the words in words.txt are
	// already 5-characters long, so we don't
	// need to filter them.
	let words = include_str!("../words.txt").split("\n").into_iter();

	words.filter(|x| { 
		for cond in contains {
			match cond {
				Contains::No(chr) if x.chars().any(|v| &v == chr) => return false,
				Contains::Some(chr) if !x.chars().any(|v| &v == chr) => return false,
				Contains::Exact(chr, pos) if x.chars().nth(*pos).unwrap() != *chr => return false,
				Contains::NoneAt(chr, pos) if x.chars().nth(*pos).unwrap() == *chr => return false,
				_ => {}
			}
		}
		true
	 }).collect::<Vec<&str>>()
}
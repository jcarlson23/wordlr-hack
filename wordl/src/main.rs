// set up a dictionary, ordered by word length and then
// within that, letter variation, so I need a score of
// letter ferquency.
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io;
use std::io::{stdin,stdout,Write};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

// Rust can't create a static hashmap... Ah, yes, we need to use
// lazy_static crate (lib)

pub struct Constraints {

	locations: HashMap<char,u32>,
	occurrences: HashSet<char>,
}

impl Constraints {
	pub fn new(mut self) {
		self.locations = HashMap::new();
		self.occurrences = HashSet::new();
	}
}

// english letter frequency (x100, i.e., 'a' occurs in 7.8%)
lazy_static! {
	     static ref LETTERS: HashMap<char, u32> = { HashMap::from([
	         ('a',780),
		 	 ('b',200),
		 	 ('c',400),
    		 ('d',380),
    		 ('e',1100),
    		 ('f',140),
    		 ('g',300),
    		 ('h',230),
    		 ('i',820),
    		 ('j',210),
    		 ('k',250),
    		 ('l',530),
    		 ('m',270),
    		 ('n',720),
    		 ('o',610),
    		 ('p',280),
    		 ('q',24),
    		 ('r',730),
		 	 ('s',870),
    		 ('t',670),
    		 ('u',330),
    		 ('v',100),
    		 ('w',91),
    		 ('x',27),
    		 ('y',160),
    		 ('z',44),
    		 ])
		 };
    }

fn score_letter_frequency(word: &str) -> u32 {

   let mut bag: HashSet<char> = HashSet::new();
   let mut total: u32 = 0;
   
   for c in word.chars() {
       if !bag.contains(&c) {
       	  if let val = LETTERS.get(&c) {
	     total = total + val.unwrap();
	     }
	  }
   }
   return total;
}

fn read_dictionary(dictionary: &str) -> io::Result<()> {
   println!("Reading {}!", &dictionary);

   let file = File::open(&dictionary)?;
   let mut reader = io::BufReader::new(file);
   let mut buf = String::new();
   while reader.read_line(&mut buf)? > 0 {
   	 {
		let line = buf.trim_end();
		let s_len = line.len();
		println!("{} : {}",line,s_len);
		let subdictionary = s_len.to_string() + ".txt";
		let mut file = OpenOptions::new().create(true).write(true).append(true).open(subdictionary).unwrap();
		if let Err(e) = write!(file,"{}",buf) {
			eprintln!("Couldn't write to file: {}",e);
		}
		
	 }
	 
	 buf.clear();
   }
   Ok(())
}


fn main() {
   let args: Vec<String> = env::args().collect();
   
   // if we are passed a dictionary, regenerate the sub-dicts
   if args.len() > 1 {
   	let dictionary = &args[1];
   	let result = read_dictionary( &dictionary );
   }
   
   // get the length of the word we're trying to guess.
   print!("Enter the word length to guess: ");
   let _=stdout().flush();
   
   let mut word_len = String::new();
   stdin().read_line(&mut word_len).expect("Did not enter a number [1-20]");
   println!("You typed: {}",word_len);
   
   let subdictname = word_len + ".txt";
   let mut dict = OpenOptions::new().read(true).open(subdictname);
   
   // now we need some game logic to iterate through guesses
   
   
}
// set up a dictionary, ordered by word length and then
// within that, letter variation, so I need a score of
// letter ferquency.
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

// Rust can't create a static hashmap... Ah, yes, we need to use
// lazy_static crate (lib)

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
	 }
	 buf.clear();
   }
   Ok(())
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let dictionary = &args[1];
   
   let result = read_dictionary( &dictionary );
   
}
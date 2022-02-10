// set up a dictionary, ordered by word length and then
// within that, letter variation, so I need a score of
// letter ferquency.
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::io::{stdin,stdout,BufRead,BufReader,Write};
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::collections::HashSet;
use lazy_static::lazy_static;

// Rust can't create a static hashmap... Ah, yes, we need to use
// lazy_static crate (lib)

struct IndexScore {
	score:u32,
	location:usize,
}

struct Constraints {

	locations: HashMap<char,usize>,
	occurrences: HashSet<char>,
	excluded: HashSet<char>,
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

// we always to maximize variation of letters
   let mut bag: HashSet<char> = HashSet::new();
   let mut total: u32 = 0;
   
   for c in word.chars() {
       if !bag.contains(&c) {
       	  if let val = LETTERS.get(&c) {
	     total = total + val.unwrap();
	     bag.insert(c); 
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

// grabbed utility function of the web
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn generate_guess<'a>( wordlist:&Vec<String>, constraint:&Constraints) -> Vec<String> {
	
	let mut for_consideration:HashSet<usize>= HashSet::new();
	
	// zeroth, if the word has any letters we've ruled out
	// first filter the word list subject to the constraints
	for (index, word) in wordlist.iter().enumerate() {
	
		let mut exclusion_flag = false;
		
		for ex in &constraint.excluded {
			// println!("looking for {} in {}",ex,word);
			if word.contains(&ex.to_string()) {
				// println!("Excluding {}",word);
				exclusion_flag = true;
				continue // the word is not a candidate
			}
		}
		
		if exclusion_flag {
			continue
		}
		
		// first check that the word contains a given letter
		for letter in word.chars() {
			if constraint.occurrences.contains(&letter) {
				for_consideration.insert(index.try_into().unwrap());
			}
		}
		
		// secondly, check a letter at a given position
		let chrs = word.as_bytes();
		for (&c,&i) in &constraint.locations {
			let ch = c as u8;
			if chrs[i] != ch {
				exclusion_flag = true;
				continue
			}
		}
		
		if exclusion_flag {
			continue
		}
		
		// it's passed our location
		for_consideration.insert(index.try_into().unwrap());
	}
	
	let mut high_score:u32 = 0;
	let mut candidates:HashSet<usize> = HashSet::new();
	let mut cands:Vec<IndexScore> = Vec::new();
	
	// ok, now we score our words for consideration
	for cons in for_consideration {
		let score = score_letter_frequency( &wordlist[cons] );
		
		let mut is = IndexScore {
			score: score,
			location: cons,
		};
		
		cands.sort_by_key(|k| k.score );
		
		let mut high_score = 0;
		
		if cands.len() > 5 {
			high_score = cands[0].score;
			cands.remove(0);
		}
		
		if score > high_score {
			is.score = score;
			
			cands.insert(0, is);
			
			candidates.clear();
			candidates.insert(cons);
		}
		else if score == high_score {
			candidates.insert(cons);
		}
	}
	
	let mut guesslist: Vec<String> = Vec::new();
	
	
	
	for c in cands {
		
		if c.location < wordlist.len() {
			guesslist.insert(0,wordlist[c.location].clone());
		}
		else {
			println!("Error creating guess list");
		}
		// con = wordlist[c].clone();
		// return con;
	}
	
	return guesslist;
	
}

fn guess_feedback(word:String) -> (HashMap<char,usize>,HashSet<char>,HashSet<char>) {
	
	println!("For each letter, enter\n (1) Not found \n(2) Found but we don't know where\n(3) Found at the given location\n");
	let mut index:usize = 0;
	
	let mut locations = HashMap::new();
	let mut occurrences = HashSet::new();
	let mut excluded = HashSet::new();
		
	for ch in word.chars() {
		println!("(1) Not found \n(2) Found but we don't know where\n(3) Found at the given location\n");
		println!("Letter: {}",ch);
		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer);
		let res = buffer.trim();
		if res == "1" {
			excluded.insert(ch);
		}
		else if res == "2" {
			occurrences.insert(ch);
		} 
		else if res == "3" {
			locations.insert(ch,index);
		}
		else {
			println!("{} Not understood",res);
		}
		
		index = index + 1;
	}
	
	return (locations, occurrences, excluded);
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
   println!("Word length: {}",word_len);
   word_len.truncate(word_len.len()-1);
   
   let subdictname = word_len + ".txt";
   println!("Sub dictionary: {}", subdictname);

   
   // read the sub-dictionary into a wordlist
   let wordlist = lines_from_file(subdictname).unwrap();
   println!("Loaded {} words to work with",wordlist.len());
   
   
   // now we need some game logic to iterate through guesses
   let mut constraints = Constraints {
   		locations: HashMap::new(),
		occurrences: HashSet::new(),
		excluded: HashSet::new(),
   };
   
   for i in 0..4 {
   		println!("* * * * ROUND {} * * * *",i);
   		let guesses = generate_guess( &wordlist, &constraints );
   		for g in guesses {
   			println!("Option: {}",g);
   		}
	   	
   
   		// get what guess was used.
   		println!("Enter the guess you used");
   		let mut guess = String::new();
   		stdin().read_line(&mut guess).expect("We need a guess");
   		guess.pop();
   		
	   	// now we need to get feedback
	   	let (loc, occ, ex ) = guess_feedback(guess);
	   	// add these to our constraints
   
	   	constraints.locations.extend(&loc);
	   	constraints.occurrences.extend(&occ);
	   	constraints.excluded.extend(&ex);
   }
  
   
   
}
use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   println!("Hello!  Here are the args: {:?}",args);
   println!("The second arg is {:?}",args[1]);
   }
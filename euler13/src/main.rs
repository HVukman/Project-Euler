use std::fs::File;



fn main() {
   
	let f = File::open("number.txt");
	print!("{}",f)
	
}

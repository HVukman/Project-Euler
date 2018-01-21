// Euler 1

// use std::io;
// use std::cmp::Ordering;

fn main() {
   let mut i=1;
   let mut zaehler=0;
   while i<1000 {
        if i%3 == 0{
		   zaehler +=i;
		   }
		else if i%5 == 0 {
		   zaehler +=i;
		   }
		
		i +=1;
   }
   print!("{}",zaehler)
   
   }
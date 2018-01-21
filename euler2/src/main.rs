
//use std::io;
// use std::f64;
// use std::cmp::Ordering;

fn main() {
   let mut i=0;
   let mut zaehler=0;
 //  let x: i32 = 40;
 //  let ende=x.pow(6);
     let ende=4000000;
   while fib(i)<ende {
   
			let zahl = fib(i);
			i +=1;
			
			if zahl%2 == 0 {
			   zaehler += zahl;
			   }
   }
   print!("{}",zaehler)
   }
fn fib(x: i32) -> i32 {
    if x == 0 {
         return 0;
		      }
    else if x == 1{
         return 1; 
		         }
    else {
         return fib(x-1) + fib(x-2);
		 }
                       }
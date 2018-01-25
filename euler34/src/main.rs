
//http://www.mathblog.dk/project-euler-34-factorial-digits/
fn factor(value: i64) -> i64 {
  // Factorial function recursive
 
  if value==0 { 
		return 1;
  }
  else { 
       return value * factor(value-1); 
	   } 
 
}

fn main() {
  // Input number
  
  let mut result: (i64) =0;
  let mut i: (i64) = 10;
  
  // Limit
  let mut limit: (i64) = factor(9);
  limit = 7* limit;
 
  
  while  i<limit {
    let  mut sum_Of_Facts: (i64)  = 0;
    let  mut number: (i64)  = i;
	while number>0 {
	     let d: (i64) = number%10;
		 number = number/10;
		 sum_Of_Facts += factor(d);
	 
	 }
  // Factorial function with input_int	
    if sum_Of_Facts==i{
		result += i;
	   }
	i += 1;

  }
  println!("{}", result);
}

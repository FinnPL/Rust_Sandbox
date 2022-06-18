pub fn run(){
   // Primitive Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  // Vector non primitiv 
  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1; // vec2 is a reference to vec1 (&)

  println!("Values: {:?}", (&vec1, vec2)); 

}
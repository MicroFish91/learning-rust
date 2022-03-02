fn main() {
  // arrays
  // fixed length, length known at compile time
  // homogenous - can only contain items of the same data type
  // :[type, length]
  let array = [1, 2, 3];
  println!("{}", array[0]);

  // tuples
  // heterogenous - items can be of different types
  // fixed length, length known at compile time
  // empty tuple called a unit
  let tuple = (true, 2, 3);
  println!("{}", tuple.0);
}
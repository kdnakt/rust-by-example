fn main(){
  println!("{} days", 31);

  println!("{0}, this is {1}. {1}, this is {0}.", "Bob", "Alice");

  println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over"
  );

  println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

  println!("{number:>width$}",
    number=1,
    width=6);
  println!("{number:>0width$}",
    number=1,
    width=6);
  
  //println!("{0}, my name is {1} {0}", "Bond"); /* error: invalid reference to positional argumoent 1 */
  //println!("{0}, my name is {1} {0}", "Bond", "James", "Bond"); /* error: argument never used */
  println!("{0}, my name is {1} {0}", "Bond", "James");

  #[allow(dead_code)]
  struct Structure(i32);
  //println!("This struct is `{}` won't print...", Structure(3)); /* error: main::Structure doens't implement std::fmt::Display */

  let pi = 3.141592;
  println!("Pi is roughly {:.*}", 2, pi);
}

// Ownership rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let mut s = String::from("hello");
    // s is valid from this point forward
    // Use String::from in situations where you need a mutable, growable string
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    let x = 5;
    let _y = x;
    // In this situation, both x and y are valid
    // because integers are stored on the stack and have a known, fixed size

    let s1 = String::from("hello");
    let s2 = s1;
    // String holds a pointer to the data, length, and capacity
    // When we assign s1 to s2, the pointer, length, and capacity are
    // copied to s2. Now both s1 and s2 point to the same data.
    // To prevent a double free error, Rust invalidates s1.
    // println!("{}, world!", s1); // This would cause a compile-time error
    // s1 is no longer valid here, the data has been "moved" to s2
    println!("{}, world!", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    // s3 and s4 are both valid and own their own data
    // because we used the clone method to create a deep copy of the data
    println!("s3 = {}, s4 = {}", s3, s4);

    // The types that implement the Copy trait are (meaning deep and shallow copy are the same):
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

}

fn owner_code() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn secondary() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn return_tuple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
} // We do this because when s1 is passed to calculate_length, it
  // moves into calculate_length and is no longer valid here.
  // If we tried to use s1 after this point, our code would fail to compile.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// To avoid returning ownership, we can use references and borrowing
fn use_reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s is a reference to a String
  // Because it is a reference, the String is not dropped when s goes out of scope
  // We cannot modify s because it is an immutable reference

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// You can have multiple mutable references, but only if they are in separate scopes
fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}


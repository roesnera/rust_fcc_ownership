fn main() {
    // share_reference();
    take_ownership_problem();
}

fn share_reference() {

    // the prompt is to make the following code perform as intended without consequence
    // let x = String::from("hello, world");
    // let y = x;
    // println!("{},{}",x,y);
    // the problem to solve is that by default, because Strings are stored on the heap,
    // ownership is transferred when a new variable is assigned to an old heap-stored variable
    // in order to solve this, we can simply borrow the value, as in the example below
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}",x,y);
    // or we can clone the value, which duplicates the heap values and creates a new pointer
    let a = String::from("hello, world");
    let b = a.clone();
    println!("{},{}",a,b);
}

fn take_ownership_problem() {
    // make the below work without modifying this fn
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{s2}");
}

// in order to make the take_ownership_problem fn work,
// we must return the s reference at the end of the take_ownership(s) fn
// that way, the ownership of s is returned at the end of the function call
// and not deallocated when it drops out of scope
fn take_ownership(s: String) -> String {
    println!("{s}");
    s
}

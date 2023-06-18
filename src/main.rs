fn main() {
    // share_reference();
    // take_ownership_problem();
    // give_ownership_problem();
    // clone_vs_copy();
    // mutability_ownership_transfer();
    // partial_moving();
    //  tuple_references();
    tuple_references_2();
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

fn give_ownership_problem() {
    let s = give_ownership();
    println!("{}",s);
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // must convert string into bytes, for arbitrary reason
    // let _s = s.into_bytes();
    // the above will consume the data at s in the heap, 
    // because into_bytes does not take an immutable reference, it just uses the reference in its
    // entirety
    // but a method called as_bytes() just uses a &self reference, and so we can use that instead
    // to achieve our intended effect
    let _s = s.as_bytes();
    s
}

fn clone_vs_copy() {
    // refactor the following code to be able to use .copy() instead of .clone()
    // let x = (1, 2, (), "hello".to_string());
    // let y = x;
    // println!("{}. {}",x,y);

    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    // the below will implicitly use the .copy() fn to make a new variable y with the same data in
    // the stack because all the data implements the .copy() trait
    let y : (i32, i32, (), &str) = x;
    println!("{:?}. {:?}",x,y);
}

fn mutability_ownership_transfer() {
    let s: String = String::from("hello, ");

    // the below will give the following error:
    // "cannot borrow s1 as mutable because it is not decalred as mutable"
    // let s1: String = s;
    // therefore, we do the following modification to declare s1 as mutable
    let mut s1: String = s;

    s1.push_str("world");

    println!("Success!");
}

fn partial_moving() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20)
    };
    
    // 'name' is moved out of person, but 'age' is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // the below will throw an error
    // "Error! borrow of partially moved value: 'person' partial move occurs"
    // println!("The person struct is {:?}", person);
    // 'person' cannot be moved, but 'person.age' can be used as it is not moved

    println!("the person's age: {}", person.age);
}


fn tuple_references() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;
    
    // only modify the below line to avoid the error
    // println!("{:?}", t);
    println!("{:?}", t.1);
}

// you could also achieve the following using t.clone()
// instead of using ref
fn tuple_references_2() {
    let t = (String::from("hello"), String::from("world"));

    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t);
}

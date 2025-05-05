fn main() {
    println!("Hello, world!");

    datatypes();
}

fn datatypes() {
    let x: i32 = 500;
    println!("the value of x is {}", x);

    let x: &str = "this is false";

    println!("the value of x is {}", x);

    let y = String::from("this is true");
    println!("the value of y is {}", y);

    let x: tup = (5, "this is false", 5.0);
    println!("the value of x is {:?}", x); 

}



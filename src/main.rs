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

    let w: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the value of w is {:?}", w);

    let zs: [i32; 5] = [1; 5];
    println!("the value of zs is {:?}", zs);
    let mut i = 0;
    while i < 5 {
        println!("the value of zs[{}] is {}", i, zs[i]);
        i += 1;
    }

}



fn main() {
    let x = 27;
    println!("The value of x is {}", x);

    {
        let x = 2;
        println!("The value of x is {}", x);
    }

    let x = 5;
    println!("The value of x is {}", x);
}
pub mod understanding_closures {
    pub fn what_is_a_closure() {
        let add_one = |x| x + 1;
        let a = add_one(5);
        println!("The value of a is: {}", a);

        let multiply_by_two = |x| x * 2;
        let b = multiply_by_two(5);
        println!("The value of b is: {}", b);

        let c = multiply_by_two(10);
        println!("The value of c is: {}\n", c);

        // variable capturing
        let x = 7;
        let print_x = || println!("The value of x is: {}\n", x);
        print_x();

        // regular functions
        let y = 10;
        println!("The value of y is: {}", y);

        fn add_one_fn(x: i32) -> i32 {
            x + 1
        }

        let f = add_one_fn;
        let e = f(5);
        println!("The value of e is: {}\n", e);

        // inline closures, less used but may encounter
        println!("Inline Closure: {}\n", (|x, y| x + y)(1, 2));
    }
}
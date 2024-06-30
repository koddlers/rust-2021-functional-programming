pub mod understanding_closures {
    use std::thread;

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

    pub fn closure_type_inference_and_annotation() {
        fn add_one_fn(x: i32) -> i32 {
            x + 1
        }

        // let add_one = |x| x + 1;
        // with type parameters and return type annotations explicitly declared
        let add_one = |x: i32| -> i32 { x + 1 };
        let a = add_one(5);
        println!("The value of a is (closure): {}", a);

        let b = add_one_fn(5);
        println!("The value of b is (function): {}", b);

        // closure parameter types are inferred from the first usage of the closure
        // and cannot be changed later on
        let print = |x| println!("{}", x);
        print("Hello, World");
        // now the closure `print` accepts `string` (&str) only and WILL NOT accept other types
        // like an `i32` or something else, uncommenting the following line will produce a compile
        // time error
        // print(5);
    }

    pub fn capturing_the_environment() {
        let y  = 5;
        let add_y = |x| x + y;
        let a = add_y(10);
        println!("The value of a is: {}\n", a);

        let mut y = 5;
        let mut add_y2 = |x| {
            y += x;
            y
        };

        let a = add_y2(10);
        println!("The value of a is: {}", a);
        println!("The value of y is: {}\n", y);

        // move ownership of captured variable `message` to closure `thread`
        let message = "Hello from a thread".to_string();
        let thread = thread::spawn(move || {
            println!("Message owned: {}", message);
        });
        thread.join().unwrap();

        // the following line will produce error, because `message` is now OWNED by the `thread` closure
        // println!("{}", message);
    }
}
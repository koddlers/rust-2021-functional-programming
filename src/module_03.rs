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
        let y = 5;
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

    fn call_with_one<F>(func: F) -> usize
        where F: Fn(usize) -> usize
    {
        func(1)
    }

    pub fn different_types_of_closures() {
        let x = 5;
        // `print_x` implements the trait `Fn`. This trait applies to closures that don't capture
        // variables at all, they can be called multiple times without changing their environment.
        let print_x = || println!("{}", x);
        print_x();
        print_x();
        println!();

        // another example of `Fn` trait
        let double = |x| x * 2;
        assert_eq!(call_with_one(double), 2);
        let result = call_with_one(double) == 2;
        if result {
            println!("call_with_once(double) == 2: {}\n", result);
        }

        // example of `FnMut`
        let mut x = 5;
        let mut increment_x = || x += 1;
        increment_x();
        increment_x();
        println!("x: {}", x);

        // another example of `FnMut`
        fn do_twice<F>(mut func: F) where F: FnMut() {
            func();
            func();
        }

        let mut val: usize = 1;
        {
            let add_two_to_val = || val += 2;
            do_twice(add_two_to_val);
        }
        assert_eq!(val, 5);
        println!("adding {} to {} twice, results in {}\n", 2, 1, val);

        // example of `FnOnce`
        let s = "Hello".to_string();
        let consume = move || {
            let bytes = s.into_bytes();
            println!("bytes: {:?}", bytes);
        };
        consume();

        // any call to `consume()` after once it's been called, will produce error
        // because the variable `s` has been moved to the closure and consumed
        // consume();

        // the following though, moves the value from `text` to `moved`, which just moves the ownership
        let text = "Words".to_string();
        let moved = text.into_bytes();
        println!("moved: {:?}\n", moved);
        // uncommenting the following line will produce error,
        // this can be fixed by cloning instead of moving, like this
        // let moved = text.clone().into_bytes();
        // println!("text: {:?}", text);

        // another example of `FnOnce`
        fn consume_with_relish<F>(func: F) where F: FnOnce() -> String, {
            // `func` consumes its captured variables, so it cannot be run more than once
            println!("Consumed: {}", func());
            println!("Delicious");

            // Attempting to invoke `func()` again will throw a `use of moved value` error for `func`
        }

        let z = String::from("z");
        let consume_and_return_z = move || z;
        consume_with_relish(consume_and_return_z);

        // `consume_and_return_z` can no longer be invoked at this point
        // the following call will produce error
        // consume_with_relish(consume_and_return_z);
    }

    pub fn using_closures_in_higher_order_functions() {
        // call with either value of `x`
        let x = None;
        // let x = Some(5);
        let y = x.unwrap_or_else(|| {
            let z = 10;
            z * 2
        });
        println!("{}", y);
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    pub fn using_closures_in_higher_order_functions_v2() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        println!("Before sorting:\n{:#?}\n", list);

        // use the `sort_by_key` method to order the rectangles by width
        // the closure `|r| r.width` is passed to `sort_by_key` to specify the key for sorting
        list.sort_by_key(|r| r.width);

        // print the sorted list of rectangles
        println!("After sorting:\n{:#?}", list);
    }
}
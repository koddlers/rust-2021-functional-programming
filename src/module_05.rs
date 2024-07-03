pub mod exploring_additional_topics {
    pub fn immutability() {
        // you have to explicitly declare a variable to be mutable with the `mut` keyword
        let mut x = 5;
        x = 6;
        println!("The value of x is: {x}");
    }

    pub fn higher_order_functions() {
        let numbers = vec![1, 2, 3, 4, 5];
        let closure_square = |x| x * x;

        // `map()` is a higher order function that can take other functions/closures
        // as parameters. here, `closure_square` in this case.
        let squared: Vec<_> = numbers.iter().map(closure_square).collect();

        println!("Squared: {:?}", squared);
    }

    // `multiplier` returns a function (or a closure in this case, to be precise)
    fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |number| number * factor
    }

    pub fn higher_order_functions_v2() {
        let double = multiplier(2);
        let result = double(5);

        println!("Result: {}", result);
    }
}
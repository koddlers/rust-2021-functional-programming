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

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    pub fn pattern_matching() {
        let light = TrafficLight::Red;

        match light {
            TrafficLight::Red => println!("Stop"),
            TrafficLight::Yellow => println!("Slow down"),
            TrafficLight::Green => println!("Go")
        }
    }

    // this is the code in the tutorial, but the following variant works too
    // fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    //     if denominator == 0.0 {
    //         Err("Cannot divide by zero!")
    //     } else {
    //         Ok(numerator / denominator)
    //     }
    // }

    // Changes:
    //     1. no use of lifetime specifiers (in the form of `'static`)
    //     2. use of the `String` struct instead of `str` slice (in the form of `&str`)
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("Cannot divide by zero!"))
        } else {
            Ok(numerator / denominator)
        }
    }

    pub fn result_and_option_types() {
        // use either of the `result` declarations
        let result = divide(2.0, 3.0);
        // let result = divide(2.0, 0.0);
        match result {
            Ok(res) => println!("Result: {res}"),
            Err(err) => println!("Error: {err}")
        }
    }

    pub fn result_and_option_types_v2() {
        let mut numbers = vec![1, 2, 3].into_iter();

        let first = numbers.next();     // returns Some(1)
        let second = numbers.next();    // returns Some(2)
        let third = numbers.next();     // returns Some(3)
        let fourth = numbers.next();    // returns None

        println!("{:?}", first);
        // we can call `unwrap()` here because we know a value is present in `second` by looking
        // at the `numbers` vector, BUT this is unsafe, if there were no value, it would panic
        println!("{}", second.unwrap());
        // it's rather safe to use the `debug format` instead (denoted by `{:?}`)
        println!("{:?}", second);
        println!("{third:?}");
        println!("{fourth:?}");
    }

    use std::fs;
    use std::time::Instant;

    fn search_with_loops(content: &str) -> usize {
        let mut count = 0;
        let mut word = String::new();
        let mut index = 0;

        while index < content.len() {
            let ch = content.as_bytes()[index] as char;

            if ch.is_whitespace() {
                if word == "the" {
                    count += 1;
                }
                word.clear();
            } else {
                word.push(ch);
            }

            index += ch.len_utf8();
        }

        // check for the last word in case the content doesn't end with whitespace
        if word == "the" {
            count += 1;
        }

        count
    }

    fn search_with_iterators(content: &str) -> usize {
        content
            .split_whitespace()
            .filter(|&word| word == "the")
            .count()
    }

    pub fn comparing_performance_loops_vs_iterators() {
        let content = fs::read_to_string("peace.txt").expect("Unable to read the file");

        let start = Instant::now();
        let _ = search_with_loops(&content);
        let duration = start.elapsed();
        println!("Time taken with loops: {:?}", duration);

        let start = Instant::now();
        let _ = search_with_iterators(&content);
        let duration = start.elapsed();
        println!("Time taken with iterators: {:?}", duration);
    }
}
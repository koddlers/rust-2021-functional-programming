pub mod working_with_iterators_and_lazy_evaluation {
    pub fn what_are_iterators() {
        let v = vec![1, 2, 3, 4, 5];
        let mut index = 0;

        while index < v.len() {
            println!("Item from the while loop: {}", v[index]);
            index += 1;
        }
        println!();

        for item in v.clone() {
            println!("Item from for loop: {}", item);
        }
        println!();

        v.iter().for_each(|i| println!("Item from the Iterator: {}", i));
    }

    pub fn what_are_iterators_v2() {
        let v = vec![1, 2, 3, 4, 5];

        let sum_of_squares: i32 = v.iter()
            .map(|&x| x * x)
            .sum();

        println!("Sum: {}", sum_of_squares);
    }

    pub fn what_are_iterators_v3() {
        // `.iter()` for read only immutable references
        let v = vec![1, 2, 3];
        for i in v.iter() {
            println!("Immutable for: {}", i);
        }
        println!();

        // `.iter_mut()` for mutable references
        let mut v = vec![1, 2, 3, 4, 5];
        for i in v.iter_mut() {
            *i += 10;
            println!("Mutable for: {}", i);
        }
        println!();

        // `.into_iter()` for taking ownership and consuming collection
        let v = vec![2, 4, 6, 8, 10];
        println!("original: {:?}", v);

        // since `into_iter()` consumes and takes ownership of `v`, we cannot use `v` after this point
        let result: Vec<_> = v.into_iter().map(|i| i * 20).collect();
        println!("result: {:?}", result);

        // uncommenting the following line will produce error
        // println!("original: {:?}", v);
    }

    pub fn what_are_iterators_v4() {
        let v = vec![1, 2, 3];
        let mut v_iter = v.iter();

        println!("{:?}", v_iter.next());
        println!("{:?}", v_iter.next());
        println!("{:?}", v_iter.next());
        println!("{:?}", v_iter.next());
        println!("{:?}", v_iter.next());
        println!();

        let v_iter = v.iter();
        for i in v_iter {
            println!("{:?}", i);
        }
    }
}

pub mod working_with_iterators_and_lazy_evaluation_v2 {
    #[derive(Debug)]
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 3 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    pub fn understanding_the_iterator_trait() {
        let counter = Counter::new();
        for i in counter.take(10) {
            println!("Counter is equal to {}", i);
        }
        println!();

        let counter_sum = Counter::new();
        println!("The sum of all values in `counter_sum`: {}\n", counter_sum.sum::<u32>());

        let mut counter_next = Counter::new();
        println!("The next value in `counter_next` is {}", counter_next.next().unwrap());
        println!("The next value in `counter_next` is {}\n", counter_next.next().unwrap());

        let counter = Counter::new();
        let counter_iter = counter.into_iter();
        for num in counter_iter {
            println!("Num in counter is equal to {}", num);
        }
    }
}

pub mod working_with_iterators_and_lazy_evaluation_v3 {
    pub fn consuming_adaptors() {
        let v = vec![1, 2, 3, 4, 5];
        let iter = v.iter();
        // `collect()` is a basic consuming adaptor
        let collected: Vec<_> = iter.collect();

        for item in collected {
            println!("Item is {}", item);
        }
    }

    pub fn consuming_adaptors_v2() {
        let v = vec![1, 2, 3, 4, 5];

        // `map()` is lazy, and is not executed until the iterator `v_squared` it produces is used
        let v_squared = v.iter().map(|x| {
            let result = x * x;
            println!("Squaring {} is {}", x, result);
            result
        });
        println!("Finished creating the iterator");

        let collected: Vec<_> = v_squared.collect();
        println!("Finished collecting into a vector");
        println!("{:?}", collected);
    }

    pub fn consuming_adaptors_v3() {
        let v = vec![1, 2, 3, 4, 5];
        let iter = v.iter();
        let total: i32 = iter.sum();
        println!("total is {}", total);
    }

    pub fn iterator_adaptors() {
        let v = vec![1, 2, 3, 4, 5];
        let iter = v.iter();
        let squared: Vec<_> = iter.map(|x| x * x).collect();

        for item in squared {
            println!("Item is {}", item);
        }
    }

    pub fn iterator_adaptors_v2() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = v.iter();
        let evens: Vec<_> = iter.filter(|x| *x % 2 == 0).collect();

        for even in evens {
            println!("Even number: {}", even);
        }
    }

    pub fn iterator_adaptors_v3() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = v.iter();
        let evens: Vec<_> = iter.map(|x| x * x).filter(|x| *x % 2 == 0).collect();

        for even in evens {
            println!("Even number squared: {}", even);
        }
    }

    pub fn common_methods_of_the_iterator_trait() {
        let sum = vec![1, 2, 3, 4, 5].iter().fold(0, |acc, x| acc + x);
        println!("Sum: {}\n", sum);

        // all()
        let numbers = vec![2, 4, 6, 8, 10];
        let all_even = numbers.iter().all(|&x| x % 2 == 0);
        println!("Are all numbers in the list even?: {}", all_even);

        // any()
        let any_odd = numbers.iter().any(|x| x % 2 == 1);
        println!("Is any number in the list odd?: {}\n", any_odd);

        // count()
        let total = numbers.iter().count();
        println!("numbers.count(): {}\n", total);

        // zip()
        let names = vec!["Alice", "Bob", "Marshal"];
        let scores = vec![85, 90, 99];
        let paired = names.iter().zip(scores.iter());
        for (name, score) in paired {
            println!("{name} scored {score} points");
        }
        println!();

        // chain()
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6, 7];
        let chained = first.iter().chain(second.iter());
        println!("first and second chained together: {:?}", chained);
        for item in chained {
            println!("{item}");
        }
        println!();

        // peekable()
        let mut numbers = vec![1, 2, 3, 4, 5].into_iter().peekable();
        if let Some(&first) = numbers.peek() {
            println!("First number is: {}", first);
        }
        for num in numbers {
            println!("{}", num);
        }
        println!();

        // `skip()` and `take()`
        let numbers = vec![1, 2, 3, 4, 5].into_iter();
        let skipped_and_taken = numbers.skip(1).take(3);
        println!("skipped_and_taken: {:?}", skipped_and_taken);
        for num in skipped_and_taken {
            println!("{num}");
        }
    }
}
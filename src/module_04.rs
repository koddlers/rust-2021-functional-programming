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
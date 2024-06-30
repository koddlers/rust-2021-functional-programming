pub mod introducing_functional_programming {
    pub fn what_is_functional_programming() {
        // Imperative
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        for number in numbers {
            sum += number;
        }
        println!("Sum (Imperative): {}", sum);

        // Functional
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers.iter().fold(0, |acc, x| acc + x);
        println!("Sum (Functional): {}\n", sum);

        // Imperative
        let numbers = vec![1, 2, 3, 4, 5];
        let mut evens = Vec::new();

        for num in numbers {
            if num % 2 == 0 {
                evens.push(num);
            }
        }
        println!("Even numbers (Imperative): {:?}", evens);

        // Functional
        let numbers = vec![1, 2, 3, 4, 5];
        let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
        println!("Even numbers (Functional): {:?}\n", evens);
    }
}
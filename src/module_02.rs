pub mod introducing_functional_programming {
    pub fn what_is_functional_programming() {
        // Imperative
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        for number in numbers {
            sum += number;
        }
        println!("Sum: {}", sum);

        // Functional
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers.iter().fold(0, |acc, x| acc + x);
        println!("Sum: {}\n", sum);
    }
}
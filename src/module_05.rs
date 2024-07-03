pub mod exploring_additional_topics {
    pub fn immutability() {
        // you have to explicitly declare a variable to be mutable with the `mut` keyword
        let mut x = 5;
        x = 6;
        println!("The value of x is: {x}");
    }
}
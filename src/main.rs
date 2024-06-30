#![allow(unused)]

mod module_02;
mod module_03;

use module_02::introducing_functional_programming as intro_fp;
use module_03::understanding_closures;

fn main() {
    // Module 02 - Introducing Functional Programming
    // intro_fp::what_is_functional_programming();

    // Module 03 - Understanding Closures
    // understanding_closures::what_is_a_closure();
    // understanding_closures::closure_type_inference_and_annotation();
    // understanding_closures::capturing_the_environment();
    understanding_closures::different_types_of_closures();
}

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Fix the macro call.
    my_macro!();
}

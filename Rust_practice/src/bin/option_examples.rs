fn describe(value: Option<i32>) {
    let Some(number) = value else {
        println!("No number was provided");
        return;
    };

    println!("The number is {number}");
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(number) => Some(number + 1),
    }
}

fn main() {
    describe(Some(10));
    describe(None);

    let config_max = Some(3_u8);

    if let Some(max) = config_max {
        println!("The maximum is {max}");
    } else {
        println!("No maximum value was set");
    }

    println!("Five plus one: {:?}", plus_one(Some(5)));
    println!("None plus one: {:?}", plus_one(None));
}

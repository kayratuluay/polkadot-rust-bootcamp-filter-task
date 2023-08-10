use std::{vec, io};

struct FilterCondition<T> {
    filter: T
}

impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item != &self.filter
    }
}

fn custom_filter<T: PartialOrd> (list: Vec<T>, filter: FilterCondition<T>) -> Vec<T> {
    return list.into_iter().filter(|item| filter.is_match(item)).collect();
}

fn main() {
    let numbers: Vec<u32> = vec![1, 2, 3, 5, 16, 70, 42];

    println!("Select a number to filter");
    println!("{:?}",numbers);

    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(n) => println!("{} bytes are read.", n),
        Err(error) => panic!("An error has been occured. err: {}",error),
    }

    println!("{}", user_input);

    let target_number: u32 = match user_input.trim().parse() {
        Ok(number) => number,
        Err(_e) => panic!("Parsing operation has been failed."),
    };

    let my_filter: FilterCondition<u32> = FilterCondition { filter:target_number };

    let result = custom_filter(numbers, my_filter);

    println!("The result is: {:?}", result);
}

struct FilterCondition<T> {
    filter: T
}

impl<T: PartialOrd> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        item == &self.filter
    }
}

fn custom_filter<T: PartialOrd> (list: Vec<T>, filter: FilterCondition<T>) -> Vec<T> {
    return list.into_iter().filter(|item| filter.is_match(item)).collect();
}

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>, // variables are private
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // a simple getter
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    // has the wrong average to begin with..
    let mut my_collection = AveragedCollection {
        list: vec![1, 2, 3],
        average: 0.0,
    };
    println!("{my_collection:#?}");
    println!("Hello, world! {}", my_collection.average());
    my_collection.add(4);
    println!("{my_collection:#?}");

    // there is no straight forward inheritance in rust
    // instead you can use trait objects / bounds
}

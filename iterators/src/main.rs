#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_that_fit(shoes: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    let iter = shoes.into_iter().filter(|s| s.size == my_size);
    iter.collect()
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    println!("Shoes: {shoes:?}");

    let my_size = 13;

    let shoes_that_fit_me = shoes_that_fit(shoes, my_size);

    println!("Shoes that fit me: {shoes_that_fit_me:?}");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    println!("v1_iter: {:?}", v1_iter.next());
    // note that this has moved the iterator
    // to the next element in the loop below as well
    // returns immutable references to the values in the vector

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Hello, world!");

    // iterators seem to provide a nice way to do vectorized operations
    let total: i32 = v1.iter().sum();

    println!("Total: {total}");

    // iterators are lazy, so need to call collect to actually consume the iterator.
    // collect consumes iterators and returns collection
    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();
    // Vec<_> is a type annotation for an inferred type

    println!("New vector {v2:?}");
}

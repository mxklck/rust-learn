// this is a trait object
pub trait Draw {
    fn draw(&self);
}

// don't know why we need this dynamic behavior thing
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// this works differently than using trait bounds (we store a vector of box draws...)
// If you’ll only ever have homogeneous collections,
// using generics and trait bounds is preferable because
// the definitions will be monomorphized at compile time to use the concrete types.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // dummy code
        let _ = 5;
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

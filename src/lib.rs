pub fn flatten<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}

pub struct Flatten<I> {
    outer: I,
}

impl<I> Flatten<I> {
    pub fn new(iter: I) -> Self {
        Self { outer: iter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

use std::error::Error;

pub trait ErrorTrait {
    fn external(e: Box<dyn Error>) -> Self;
}

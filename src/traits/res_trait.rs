use std::error::Error;

use super::ErrorTrait;

pub trait ResTrait {
    type Error: Error + ErrorTrait;

    fn error(e: Self::Error) -> Self;
}

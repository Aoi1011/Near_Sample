use core::panic;
use std::{fmt::Debug, ops::Add};

#[derive(Clone, Copy, Debug)]
pub struct FieldElement<T>
where
    T: Add<Output = T>,
{
    pub num: T,
    pub prime: T,
}

impl<T> FieldElement<T>
where
    T: PartialOrd + Debug + Add<Output = T>,
{
    pub fn new(num: T, prime: T) -> Self {
        if num > prime {
            panic!("Num {:?} not in field range 0 to {:?}", num, prime)
        }
        Self { num, prime }
    }
}

#[cfg(test)]
mod tests {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn new_test() {
        let _ = FieldElement::new(2, 3);
        let _ = FieldElement::new(U256::from(2), U256::from(3));
    }
}

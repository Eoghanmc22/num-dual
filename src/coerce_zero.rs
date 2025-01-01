use num_traits::Float;

use crate::DualNum;

pub trait CoerceZeroExt<F: Float> {
    fn coerce_zero(&self, threshold: Self) -> Self;
}

impl<T, F: Float> CoerceZeroExt<F> for T
where
    T: DualNum<F>,
{
    fn coerce_zero(&self, threshold: Self) -> Self {
        if self.abs() > threshold {
            self.clone()
        } else {
            Self::zero()
        }
    }
}

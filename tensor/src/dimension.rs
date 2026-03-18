pub trait Dimension {
    fn get_value(&self) -> usize;
}

pub struct Constant<const N: usize>;

impl<const N: usize> Dimension for Constant<N> {
    fn get_value(&self) -> usize {
        N
    }
}

impl Dimension for usize {
    fn get_value(&self) -> usize {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_dimension() {
        let dim = Constant::<3>;
        assert_eq!(dim.get_value(), 3);
    }

    #[test]
    fn usize_dimension() {
        let dim: usize = 3;
        assert_eq!(dim.get_value(), 3);
    }
}

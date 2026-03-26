use crate::dimension::Dimension;

pub trait Shape {
    const RANK: usize;

    fn size(&self) -> usize;
}

impl Shape for () {
    const RANK: usize = 0;

    fn size(&self) -> usize {
        0
    }
}

impl<D1: Dimension> Shape for (D1,) {
    const RANK: usize = 1;

    fn size(&self) -> usize {
        self.0.get_value()
    }
}

impl<D1: Dimension, D2: Dimension> Shape for (D1, D2,) {
    const RANK: usize = 2;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value()
    }
}

impl<D1: Dimension, D2: Dimension, D3: Dimension> Shape for (D1, D2, D3) {
    const RANK: usize = 3;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value() * self.2.get_value()
    }
}

impl<D1: Dimension, D2: Dimension, D3: Dimension, D4: Dimension> Shape for (D1, D2, D3, D4) {
    const RANK: usize = 4;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value() * self.2.get_value() * self.3.get_value()
    }
}

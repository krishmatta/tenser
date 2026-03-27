use crate::dimension::Dimension;

pub trait Shape: Copy {
    const RANK: usize;

    fn size(&self) -> usize;

    fn default_stride(&self) -> Vec<usize>;
}

impl Shape for () {
    const RANK: usize = 0;

    fn size(&self) -> usize {
        1
    }

    fn default_stride(&self) -> Vec<usize> {
        vec![]
    }
}

impl<D1: Dimension> Shape for (D1,) {
    const RANK: usize = 1;

    fn size(&self) -> usize {
        self.0.get_value()
    }

    fn default_stride(&self) -> Vec<usize> {
        vec![1]
    }
}

impl<D1: Dimension, D2: Dimension> Shape for (D1, D2,) {
    const RANK: usize = 2;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value()
    }

    fn default_stride(&self) -> Vec<usize> {
        let d2 = self.1.get_value();
        vec![d2, 1]
    }
}

impl<D1: Dimension, D2: Dimension, D3: Dimension> Shape for (D1, D2, D3) {
    const RANK: usize = 3;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value() * self.2.get_value()
    }

    fn default_stride(&self) -> Vec<usize> {
        let d2 = self.1.get_value();
        let d3 = self.2.get_value();
        vec![d2 * d3, d3, 1]
    }
}

impl<D1: Dimension, D2: Dimension, D3: Dimension, D4: Dimension> Shape for (D1, D2, D3, D4) {
    const RANK: usize = 4;

    fn size(&self) -> usize {
        self.0.get_value() * self.1.get_value() * self.2.get_value() * self.3.get_value()
    }

    fn default_stride(&self) -> Vec<usize> {
        let d2 = self.1.get_value();
        let d3 = self.2.get_value();
        let d4 = self.3.get_value();
        vec![d2 * d3 * d4, d3 * d4, d4, 1]
    }
}

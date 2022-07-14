use std::ops;


pub struct Array2D<T: Default + Clone> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Array2D<T> {
        let mut data = Vec::with_capacity(width * height);
        data.resize(width * height, T::default());
        Array2D {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default + Clone> ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + self.width * index.1]
    }
}

impl<T: Default + Clone> ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + self.width * index.1]
    }
}

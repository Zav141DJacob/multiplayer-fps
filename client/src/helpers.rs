use std::slice;

pub trait AsArrays {
    type Item;

    fn as_arrays<const LEN: usize>(&self) -> &[[Self::Item; LEN]];

    fn as_arrays_mut<const LEN: usize>(&mut self) -> &mut [[Self::Item; LEN]];
}

impl<T> AsArrays for [T] {
    type Item = T;

    fn as_arrays<const LEN: usize>(&self) -> &[[Self::Item; LEN]] {
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const [T; LEN], self.len() / LEN)
        }
    }

    fn as_arrays_mut<const LEN: usize>(&mut self) -> &mut [[Self::Item; LEN]] {
        unsafe {
            slice::from_raw_parts_mut(self.as_mut_ptr() as *mut [T; LEN], self.len() / LEN)
        }
    }
}

pub trait FlatArrays {
    type Item;

    fn flat_arrays(&self) -> &[Self::Item];

    fn flat_arrays_mut(&mut self) -> &mut [Self::Item];
}

impl<T, const LEN: usize> FlatArrays for [[T; LEN]] {
    type Item = T;

    fn flat_arrays(&self) -> &[Self::Item] {
        unsafe {
            slice::from_raw_parts(self.as_ptr() as *const Self::Item, self.len() * LEN)
        }
    }

    fn flat_arrays_mut(&mut self) -> &mut [Self::Item] {
        unsafe {
            slice::from_raw_parts_mut(self.as_mut_ptr() as *mut Self::Item, self.len() * LEN)
        }
    }
}

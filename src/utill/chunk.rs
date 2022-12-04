pub struct Chunk<I, const A: usize> {
    iter: I,
}

impl<I, T: Copy, const A: usize> Chunk<I, A>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, T, const A: usize> Iterator for Chunk<I, A>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    type Item = [T; A];
    fn next(&mut self) -> Option<Self::Item> {
        let mut arr: Self::Item = [T::default(); A];
        for v in arr.iter_mut() {
            *v = self.iter.next()?
        }
        Some(arr)
    }
}

pub trait ChunkIterator<T: Copy + Default, const A: usize>: Iterator<Item = T> + Sized {
    fn chunk(self) -> Chunk<Self, A> {
        Chunk::new(self)
    }
}

impl<T: Copy + Default, I: Iterator<Item = T>, const A: usize> ChunkIterator<T, A> for I {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chunk() {
        let input = "1 2 3 4 5 6 7 8 9";
        let answer = "1,2,3|4,5,6|7,8,9";
        let a = input
            .split(' ')
            .chunk()
            .map(|arr: [_; 3]| arr.join(","))
            .reduce(|acc, v| format!("{}|{}", acc, v))
            .unwrap();
        assert_eq!(answer.to_string(), a);
    }
}

pub struct Chunk<I, const A: usize> {
    iter: I,
}

impl<I, T, const A: usize> Chunk<I, A>
where
    I: Iterator<Item = T>,
    T: Default,
{
    fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, T, const A: usize> Iterator for Chunk<I, A>
where
    I: Iterator<Item = T>,
    T: Default + Copy,
{
    type Item = [T; A];
    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [T::default(); A];
        for v in arr.iter_mut() {
            *v = self.iter.next()?;
        }
        Some(arr)
    }
}

pub trait ChunkIterator<T: Default>: Iterator<Item = T> + Sized {
    fn chunk<const A: usize>(self) -> Chunk<Self, A> {
        Chunk::new(self)
    }
}

impl<T: Default, I: Iterator<Item = T>> ChunkIterator<T> for I {}

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

    #[derive(Default, Clone, Copy)]
    struct Foo {
        a: i32,
        b: i32,
    }

    #[test]
    fn test_chunk_non_mut() {
        let lst = [
            Foo::default(),
            Foo::default(),
            Foo::default(),
            Foo::default(),
        ];
        // let a = lst.iter().chunk::<3>().collect::<Vec<_>>();
    }
}

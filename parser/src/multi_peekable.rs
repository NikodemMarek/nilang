use std::collections::VecDeque;

pub struct MultiPeekable<I: Iterator> {
    iter: I,
    peeked: VecDeque<I::Item>,
}

impl<I: Iterator> Iterator for MultiPeekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.peeked.pop_front().or_else(|| self.iter.next())
    }
}

impl<I: Iterator> MultiPeekable<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            peeked: Default::default(),
        }
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        let to_peek = (n + 1).saturating_sub(self.peeked.len());
        let peeked = (&mut self.iter).take(to_peek);
        self.peeked.extend(peeked);
        self.peeked.get(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::multi_peekable::MultiPeekable;

    #[test]
    fn test_multi_peekable() {
        let mut peekable = MultiPeekable::new([1, 2, 3].iter());

        assert_eq!(**peekable.peek().unwrap(), 1);
        assert_eq!(**peekable.peek().unwrap(), 1);
        assert_eq!(*peekable.next().unwrap(), 1);
        assert_eq!(**peekable.peek_nth(1).unwrap(), 3);
        assert_eq!(**peekable.peek_nth(0).unwrap(), 2);
        assert_eq!(*peekable.next().unwrap(), 2);
        assert_eq!(*peekable.next().unwrap(), 3);
        assert_eq!(peekable.peek(), None);
    }
}

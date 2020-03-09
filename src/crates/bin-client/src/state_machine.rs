pub struct Alternator<'a, T> {
    pair: [&'a T; 2],
    next_index: usize,
}

impl<'a, T> Alternator<'a, T> {
    pub fn new(first: &'a T, second: &'a T) -> Self {
        Alternator {
            pair: [first, second],
            // first call to "next" will return index 0
            next_index: 1,
        }
    }

    pub fn next(&mut self) -> &T {
        self.next_index ^= 1;
        self.pair[self.next_index]
    }
}

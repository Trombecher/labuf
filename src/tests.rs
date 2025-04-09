#![cfg(test)]

use super::*;
use fallible_iterator::IteratorExt;

#[test]
fn peek_n() {
    let mut lab = [1, 2, 3, 4, 5].into_iter().into_fallible().buffered();

    assert_eq!(lab.peek(), Ok(Some(&1)));
    assert_eq!(lab.peek_n(0), Ok(Some(&1)));
    assert_eq!(lab.peek_n(0), Ok(Some(&1)));
    assert_eq!(lab.peek_n(1), Ok(Some(&2)));
    assert_eq!(lab.peek_n(2), Ok(Some(&3)));
    assert_eq!(lab.peek_n(3), Ok(Some(&4)));
    assert_eq!(lab.peek_n(4), Ok(Some(&5)));
    assert_eq!(lab.peek_n(5), Ok(None));
    assert_eq!(lab.peek_n(423423), Ok(None));
}

#[test]
fn advance() {
    let mut lab = [1, 2, 3, 4, 5].into_iter().into_fallible().buffered();

    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.peek(), Ok(Some(&2)));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.peek(), Ok(Some(&3)));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.advance(), Ok(()));
    assert_eq!(lab.peek(), Ok(None));
}

#[test]
fn peek_multiple() {
    let mut lab = [1, 2, 3, 4, 5].into_iter().into_fallible().buffered();

    assert_eq!(
        lab.peek_multiple::<6>(),
        Ok([Some(&1), Some(&2), Some(&3), Some(&4), Some(&5), None])
    );
}

#[cfg(feature = "allocator_api")]
#[cfg(test)]
mod alloc_tests {
    use crate::LookaheadBuffer;
    use alloc::alloc::Global;
    use fallible_iterator::IteratorExt;

    #[test]
    fn new_in() {
        let _ = LookaheadBuffer::new_in([1, 2, 3, 4, 5].into_iter().into_fallible(), Global);
    }
}

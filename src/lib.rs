#![no_std]
#![forbid(unsafe_code)]

//! # LABuf
//!
//! A lookahead-buffer implementation for [fallible_iterator]. Commonly needed and used for
//! lexers and parsers.
//!
//! This crate is `no_std`, but uses `alloc`.
//!
//! ## Usage Example
//!
//! ```
//! use fallible_iterator::{FallibleIterator, IteratorExt};
//! use labuf::{Buffered, LookaheadBuffer};
//!
//! fn main() {
//!     let mut lab = [0, 1, 2, 3, 4].into_iter()
//!         .into_fallible()
//!         .buffered();
//!
//!     assert_eq!(lab.peek(), Ok(Some(&0)));
//!     assert_eq!(lab.peek(), Ok(Some(&0)));
//!
//!     lab.next();
//!
//!     assert_eq!(lab.peek(), Ok(Some(&1)));
//!
//!     assert_eq!(lab.peek_n(3), Ok(Some(&4)));
//!     assert_eq!(lab.peek_multiple::<3>(), Ok([Some(&1), Some(&2), Some(&3)]));
//! }
//! ```

mod tests;

extern crate alloc;

use alloc::collections::VecDeque;
use fallible_iterator::FallibleIterator;

/// Helper trait to add a function to [FallibleIterator].
/// Exposes [Buffered::buffered] with default implementation.
/// This trait is already implemented for all fallible iterators.
pub trait Buffered: FallibleIterator + Sized {
    #[inline]
    fn buffered(self) -> LookaheadBuffer<Self> {
        LookaheadBuffer::new(self)
    }
}

impl<T: FallibleIterator> Buffered for T {}

/// A lookahead-buffer implementation for [fallible_iterator].
/// Allows peeking into a [FallibleIterator].
///
/// Consumes the iterator lazily, only if the queue is empty and items are needed for peeking.
pub struct LookaheadBuffer<I: FallibleIterator> {
    iter: I,
    queue: VecDeque<I::Item>,
}

impl<I: FallibleIterator> LookaheadBuffer<I> {
    /// Create a new, empty [LookaheadBuffer].
    #[inline]
    #[must_use]
    pub const fn new(iter: I) -> Self {
        Self {
            iter,
            queue: VecDeque::new(),
        }
    }

    /// Destructure `self` into the [FallibleIterator] and [VecDeque].
    #[inline]
    #[must_use]
    pub fn destructure(self) -> (I, VecDeque<I::Item>) {
        let Self { queue, iter } = self;
        (iter, queue)
    }

    /// Returns a reference to the queue.
    #[inline]
    #[must_use]
    pub const fn queue(&self) -> &VecDeque<I::Item> {
        &self.queue
    }

    /// Returns a mutable reference to the queue.
    #[inline]
    #[must_use]
    pub const fn queue_mut(&mut self) -> &mut VecDeque<I::Item> {
        &mut self.queue
    }

    /// Returns a reference to the underlying iterator.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> &I {
        &self.iter
    }

    /// Returns a mutable reference to the underlying iterator.
    #[inline]
    #[must_use]
    pub const fn iter_mut(&mut self) -> &mut I {
        &mut self.iter
    }

    /// Tries to ensure that `n` items are in the queue. If, after a call to this function,
    /// this is not the case, then this function could not pull any more items from the iterator.
    #[inline]
    fn try_ensure(&mut self, n: usize) -> Result<(), I::Error> {
        for _ in 0..n.saturating_sub(self.queue.len()) {
            if let Some(token) = self.iter.next()? {
                self.queue.push_back(token);
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Peeks into the next `N` items. If less than `N` items will be yielded by the iterator
    /// (or are already partially yielded into the queue), then the remaining slots in the
    /// array will be [None].
    pub fn peek_multiple<const N: usize>(&mut self) -> Result<[Option<&I::Item>; N], I::Error> {
        self.try_ensure(N)?;
        let mut pack = [None; N];

        for i in 0..N {
            pack[i] = self.queue.get(i);
        }

        Ok(pack)
    }

    /// Peeks into the next `N` items, mutably. If less than `N` items will be yielded by the iterator
    /// (or are already partially yielded into the queue), then the remaining slots in the
    /// array will be [None].
    pub fn peek_multiple_mut<const N: usize>(
        &mut self,
    ) -> Result<[Option<&mut I::Item>; N], I::Error> {
        self.try_ensure(N)?;

        let mut pack = [const { None }; N];

        let mut iter = pack.iter_mut();

        for x in &mut self.queue {
            // This shoud not generate a panic.
            *iter.next().unwrap() = Some(x);
        }

        Ok(pack)
    }

    /// Peeks into the next item. Does not advance. Equivalent to `self.peek_n(0)`.
    #[inline]
    pub fn peek(&mut self) -> Result<Option<&I::Item>, I::Error> {
        self.peek_n(0)
    }

    /// Peeks into the next item, mutably. Does not advance. Equivalent to `self.peek_n_mut(0)`.
    #[inline]
    pub fn peek_mut(&mut self) -> Result<Option<&mut I::Item>, I::Error> {
        self.peek_n_mut(0)
    }

    /// Peeks into the nth item, with n=0 being the next item.
    #[inline]
    pub fn peek_n(&mut self, n: usize) -> Result<Option<&I::Item>, I::Error> {
        self.try_ensure(n + 1)?;
        Ok(self.queue.get(n))
    }

    /// Peeks into the nth item, mutably, with n=0 being the next item.
    #[inline]
    pub fn peek_n_mut(&mut self, n: usize) -> Result<Option<&mut I::Item>, I::Error> {
        self.try_ensure(n + 1)?;
        Ok(self.queue.get_mut(n))
    }

    /// Consumes the next item, returning it.
    #[inline]
    pub fn next(&mut self) -> Result<Option<I::Item>, I::Error> {
        match self.queue.pop_front() {
            Some(token) => Ok(Some(token)),
            None => self.iter.next(),
        }
    }
}

impl<T: Clone, I: FallibleIterator<Item = T> + Clone> Clone for LookaheadBuffer<I> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            iter: self.iter.clone(),
        }
    }
}

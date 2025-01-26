//! Borrowing ref newtypes from arena.

fn act(
	arena: &mut Arena
) {
    let first = arena.add(rand::random());				// ==>	+ '1
    // println!("First value: {}", arena[first]);		//		|
    let second = arena.add(rand::random());				// ==>	+ '2?
    // println!("First value: {}", arena[first]);		//		|
    // println!("Second value: {}", arena[second]);		//		|
    arena.gc();											// <==	- '1, '2?
    // println!("First value: {}", arena[first]);		// ?
    // println!("Second value: {}", arena[second]);		// ?







}

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

/// Reference into Arena-allocated u32 with a lifetime.
#[derive(Clone, Copy)]
struct ArenaRef<'a>(usize, PhantomData<&'a u32>);

/// Make it possible to use indexing.
impl Index<ArenaRef<'_>> for Arena {
    type Output = u32;

    fn index(&self, index: ArenaRef<'_>) -> &Self::Output {
        self.0.index(index.0)
    }
}

impl Arena {
	/// Add a value to arena and return its reference as ArenaRef.
	fn add(&mut self, value: u32) -> ArenaRef {
	    self.0.push(value);
	    ArenaRef(self.0.len() - 1, PhantomData)
	}

	fn gc(&mut self) {
	    self.0.retain(|_| rand::random::<bool>());
	}
}










impl ArenaRef<'_> {
    /// Forcibly release the borrow on Token.
    #[allow(dead_code)]
    fn unbind(self) -> ArenaRef<'static> {
        unsafe { std::mem::transmute::<ArenaRef, ArenaRef<'static>>(self) }
    }
}

pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    act(&mut arena);
}

use std::{marker::PhantomData, ops::Index};

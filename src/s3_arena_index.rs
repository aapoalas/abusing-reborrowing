//! Index newtype into arena.

fn act(arena: &mut Arena) {
    let first = arena.add(rand::random());				// ==>	+ '1
    // println!("First value: {}", vec[first]);			//		|
    let second = arena.add(rand::random());				// ==>	+ '2?
    // println!("First value: {}", vec[first]);			//		|
    // println!("Second value: {}", vec[second]);		//		|
    arena.gc();											// <==	- '1, '2?
    // println!("First value: {}", vec[first]);			// ?
    // println!("Second value: {}", vec[second]);		// ?
    // This line intentionally left blank.
}

/// Newtype wrapper around u32 data stored in a vector.
#[derive(Clone)]
struct Arena(Vec<u32>);

/// Index into Arena with a lifetime.
#[derive(Clone, Copy)]
struct ArenaIndex<'a>(usize, PhantomData<&'a u32>);

/// Make it possible to use indexing.
impl Index<ArenaIndex<'_>> for Arena {
    type Output = u32;

    fn index(&self, index: ArenaIndex<'_>) -> &Self::Output {
        self.0.index(index.0)
    }
}

impl Arena {
	/// Add a value to vector and return its index as VecIndex.
	fn add(&mut self, value: u32) -> ArenaIndex {
	    self.0.push(value);
	    ArenaIndex(self.0.len() - 1, PhantomData)
	}

	fn gc(&mut self) {
	    self.0.retain(|_| rand::random::<bool>());
	}
}










impl ArenaIndex<'_> {
    /// Forcibly release the borrow on Token.
    #[allow(dead_code)]
    fn unbind(self) -> ArenaIndex<'static> {
        unsafe { std::mem::transmute::<ArenaIndex, ArenaIndex<'static>>(self) }
    }
}

pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    act(&mut arena);
}

use std::{marker::PhantomData, ops::Index};

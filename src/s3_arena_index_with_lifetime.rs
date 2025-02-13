//! Step 3. Borrowing ArenaRef newtypes from Arena
//!
//! Maybe this will help?

fn act(
	arena: &mut Arena
) {
    let first = arena.get();							// ==>	+ '1
    // println!("First value: {}", arena[first]);		//		|
    let second = arena.get();							// ==>	+ '2?
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
	/// Get an ArenaRef.
	fn get(&mut self) -> ArenaRef {
	    self.0.push(rand::random());
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

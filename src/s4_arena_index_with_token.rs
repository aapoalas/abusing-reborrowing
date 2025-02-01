//! Step 4. Borrowing ArenaRefs with a separate marker lifetime
//!
//! Hey, this seems like it might work!

fn act<'a>(
	arena: &mut Arena, token: &'a mut Token
) {														// ==>				+ 'a
    let first = arena.add(rand::random(), token);		// ==>	+ '1		|
    println!("First value: {}", arena[first]);			//		|			|
    let second = arena.add(rand::random(), token);		// ==>	+ '2		|
    println!("First value: {}", arena[first]);			//		|			|
    println!("Second value: {}", arena[second]);		//		|			|
    arena.gc(token);									// <==	- '1, '2	| &'a mut
    // println!("First value: {}", arena[first]);		//					|
    // println!("Second value: {}", arena[second]);		//					|
														//					|
														//					|
														//					|
														//					|
														//					|
														//					|
														//					|
}														// <==				-

/// Marker ZST, exclusive access to the Token is required to clean the Arena.
struct Token();

impl Arena {
	/// Add a value to arena and return its references as ArenaRef, bound to a
	/// shared borrow of Token.
	fn add<'a>(&mut self, value: u32, _: &'a Token) -> ArenaRef<'a> {
	    self.0.push(value);
	    ArenaRef(self.0.len() - 1, PhantomData)
	}

	/// Clean the arena of unwanted values, requiring exclusive access to Token.
	fn gc(&mut self, _: &mut Token) {
	    self.0.retain(|_| rand::random::<bool>());
	}
}














pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    let mut token = Token();
    act(&mut arena, &mut token);
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

impl ArenaRef<'_> {
	/// Forcibly release the borrow on Token.
    fn unbind(self) -> ArenaRef<'static> {
        unsafe { std::mem::transmute::<ArenaRef, ArenaRef<'static>>(self) }
    }
}

use std::{marker::PhantomData, ops::Index};

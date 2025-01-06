//! Replacing Token reference with a ZST.

fn act<'a>(
	arena: &mut Arena, mut token: ExclusiveToken<'a>
) {														// ==>				+ 'a
    let first = arena.add(								//					|
    	rand::random(), token.shared()					//					|
    );													// ==>	+ '1		|
    let second = arena.add(								//		|			|
    	rand::random(), token.shared()					//		|			|
    );													// ==>	+ '2		|
    let third = act_two(								//		|			|
        arena, first.unbind(), second.unbind(),			//		|			|
        token.reborrow()								// <==	- '1, '2	| &'a mut
    ).unbind().bind(token.shared());					// ==>	+ '3		|
    println!("Third value: {}", arena[third]);			//		|			|
    arena.add(rand::random(), token.shared());			//		|			|
    println!("Third value: {}", arena[third]);			// 		|			|
    arena.gc(token);									// <==	- '3		| &'a mut
    // println!("Third value: {}", arena[third]);		// 					|
}														// <==				-

fn act_two<'a>(
	arena: &mut Arena,
	first: ArenaIndex,
	second: ArenaIndex,
	mut token: ExclusiveToken<'a>
) -> ArenaIndex<'a> {									// ==>				+ 'a
	let first = first.bind(token.shared());				// ==>	+ '1		|
	let second = second.bind(token.shared());			// ==>	+ '2		|
    println!("First value: {}", arena[first]); 			//		|			|
    println!("Second value: {}", arena[second]);		//		|			|
    arena.gc(token.reborrow());						// <==	- '1, '2	|
    let third = arena.add(								//					|
    	rand::random(), token.into_shared()				// <==				x
    );													// <==	  '3 == 'a	|
    third												//					|
}														// <==				-

/// Exclusive access marker, required to clean the Arena.
///
/// Only one ExclusiveToken should ever exist at any given time. Creating one
/// should be considered an unsafe action, permissible only if the caller has
/// exclusive access to the Arena.
struct ExclusiveToken<'a>(PhantomData<&'a mut ()>);

impl<'a> ExclusiveToken<'a> {
	/// Create an exclusive access marker.
	///
	/// ### Safety
	///
	/// The caller must have exclusive access to Arena. The Arena is not taken as
	/// a parameter because we do not want to keep the borrow lifetime.
	unsafe fn create() -> Self {
		Self(Default::default())
	}

	/// Reborrow the ExclusiveToken from an existing one. Use this when calling
	/// methods that may clean the Arena.
	fn reborrow(&mut self) -> ExclusiveToken {
		ExclusiveToken(Default::default())
	}

	/// Reborrow the ExclusiveToken as SharedToken. Use this when calling methods
	/// that will never clean the Arena.
	fn shared(&self) -> SharedToken {
		SharedToken::create(self)
	}

	/// Turn the ExclusiveToken into a SharedToken. Use this when needing to
	/// return values from methods that are created by a call that will never
	/// clean the Arena, or that must be manually bound to a SharedToken before
	/// returning.
	fn into_shared(self) -> SharedToken<'a> {
		SharedToken(Default::default())
	}
}

/// Shared access marker, required for working with the Arena.
///
/// Creating a SharedToken is only allowed through borrowing an ExclusiveToken.
struct SharedToken<'a>(PhantomData<&'a ()>);

impl SharedToken<'_> {
	/// Create a SharedToken from a borrow on an ExclusiveToken.
	fn create(token: &ExclusiveToken) -> Self {
		SharedToken(Default::default())
	}
}

#[derive(Clone)]
struct Arena(Vec<u32>);

impl Arena {
    /// Add a value to arena and return its index as VecIndex, bound to a shared
    /// borrow of Token.
    fn add<'a>(&mut self, value: u32, _: SharedToken<'a>) -> ArenaIndex<'a> {
        self.0.push(value);
        ArenaIndex(self.0.len() - 1, PhantomData)
    }

    /// Clean the arena of unwanted values, requiring exclusive access to Token.
    fn gc(&mut self, _: ExclusiveToken) {
        self.0.retain(|_| rand::random::<bool>());
    }
}

pub(crate) fn start() {
    let mut vec = Arena(vec![0, 1, 2, 3, 4, 5]);
    // SAFETY: We have exclusive access to Arena.
    let mut token = unsafe { ExclusiveToken::create() };
    act(&mut vec, token);
}

#[derive(Clone, Copy)]
struct ArenaIndex<'a>(usize, PhantomData<&'a u32>);

impl ArenaIndex<'_> {
	/// Bind the ArenaIndex to shared a Token borrow.
    fn bind<'a>(self, _: SharedToken<'a>) -> ArenaIndex<'a> {
        unsafe { std::mem::transmute::<ArenaIndex, ArenaIndex<'a>>(self) }
    }

	/// Forcibly release the borrow on Token.
    fn unbind(self) -> ArenaIndex<'static> {
        unsafe { std::mem::transmute::<ArenaIndex, ArenaIndex<'static>>(self) }
    }
}

impl Index<ArenaIndex<'_>> for Arena {
    type Output = u32;

    fn index(&self, index: ArenaIndex<'_>) -> &Self::Output {
        self.0.index(index.0)
    }
}

use std::{marker::PhantomData, ops::Index};

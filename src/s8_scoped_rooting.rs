//! Scoped rooting of newtype indexes during Arena cleaning.

fn act<'a, 'b>(
	arena: &mut Arena, mut token: ExclusiveToken<'a, 'b>
) {														// ==>				+ 'a, 'b
    let first = arena.add(								//					|
    	rand::random(), token.shared()					//					|
    );													// ==>	+ '1		|
    println!("First value: {}", arena[first]);			//		|			|
    let first = first.scope(arena, token.shared());		// <==	- '1		| &'b
    arena.gc(token.reborrow());							//					| &'a mut
    let first = first.get(arena, token.shared());		// ==>	+ '1		| &'a
    println!("First value: {}", arena[first]);			//		|			|
    // These lines intentionally left blank.			//					|
    //													//					|
    //													//					|
    //													//					|
    //													//					|
    //													//					|
    //													//					|
}														// <==	- '1		- 'a, 'b

/// Exclusive access marker, required to clean the Arena.
///
/// Only one ExclusiveToken should ever exist at any given time. Creating one
/// should be considered an unsafe action, permissible only if the caller has
/// exclusive access to the Arena.
struct ExclusiveToken<'a, 'b>(PhantomData<&'a mut ()>, PhantomData<&'b ()>);

impl<'a, 'b> ExclusiveToken<'a, 'b> {
	/// Create an exclusive access marker.
	///
	/// ### Safety
	///
	/// The caller must have exclusive access to Arena. The Arena is not taken as
	/// a parameter because we do not want to keep the borrow lifetime.
	unsafe fn create() -> Self {
		Self(Default::default(), Default::default())
	}

	/// Reborrow the ExclusiveToken from an existing one. Use this when calling
	/// methods that may clean the Arena.
	fn reborrow(&mut self) -> ExclusiveToken<'_, 'b> {
		ExclusiveToken(Default::default(), Default::default())
	}

	/// Reborrow the ExclusiveToken as SharedToken. Use this when calling methods
	/// that will never clean the Arena.
	fn shared(&self) -> SharedToken<'_, 'b> {
		SharedToken::create(self)
	}

	/// Turn the ExclusiveToken into a SharedToken. Use this when needing to
	/// return values from methods that are created by a call that will never
	/// clean the Arena, or that must be manually bound to a SharedToken before
	/// returning.
	fn into_shared(self) -> SharedToken<'a, 'b> {
		SharedToken(Default::default(), Default::default())
	}
}

/// Shared access marker, required for working with the Arena.
///
/// Creating a SharedToken is only allowed through borrowing an ExclusiveToken.
#[derive(Clone, Copy)]
struct SharedToken<'a, 'b>(PhantomData<&'a ()>, PhantomData<&'b ()>);

impl<'a> SharedToken<'_, 'a> {
	/// Create a SharedToken from a borrow on an ExclusiveToken.
	fn create(token: &ExclusiveToken<'_, 'a>) -> Self {
		SharedToken(Default::default(), Default::default())
	}
}

#[derive(Clone)]
struct Arena(Vec<u32>, Vec<u32>);

impl Arena {
    /// Add a value to arena and return its index as VecIndex, bound to a shared
    /// borrow of Token.
    fn add<'a>(&mut self, value: u32, _: SharedToken<'a, '_>) -> ArenaIndex<'a> {
        self.0.push(value);
        ArenaIndex(self.0.len() - 1, PhantomData)
    }

    /// Clean the arena of unwanted values, requiring exclusive access to Token.
    fn gc(&mut self, _: ExclusiveToken) {
        self.0.retain(|_| rand::random::<bool>());
    }
}

pub(crate) fn start() {
    let mut vec = Arena(vec![0, 1, 2, 3, 4, 5], vec![]);
    // SAFETY: We have exclusive access to Arena.
    let mut token = unsafe { ExclusiveToken::create() };
    act(&mut vec, token);
}

#[derive(Clone, Copy)]
struct ArenaIndex<'a>(usize, PhantomData<&'a u32>);

impl ArenaIndex<'_> {
	/// Bind the ArenaIndex to shared a Token borrow.
    fn bind<'a>(self, _: SharedToken<'a, '_>) -> ArenaIndex<'a> {
        unsafe { std::mem::transmute::<ArenaIndex, ArenaIndex<'a>>(self) }
    }

	/// Forcibly release the borrow on Token.
    fn unbind(self) -> ArenaIndex<'static> {
        unsafe { std::mem::transmute::<ArenaIndex, ArenaIndex<'static>>(self) }
    }

    fn scope<'a>(self, arena: &mut Arena, token: SharedToken<'_, 'a>) -> ScopedArenaIndex<'a> {
    	arena.1.push(*arena.0.index(self.0));
    	ScopedArenaIndex(arena.1.len() - 1, Default::default())
    }
}

impl Index<ArenaIndex<'_>> for Arena {
    type Output = u32;

    fn index(&self, index: ArenaIndex<'_>) -> &Self::Output {
        self.0.index(index.0)
    }
}

struct ScopedArenaIndex<'a>(usize, PhantomData<&'a u32>);

impl ScopedArenaIndex<'_> {
	fn get<'a>(self, arena: &mut Arena, token: SharedToken<'a, '_>) -> ArenaIndex<'a> {
		arena.add(*arena.1.index(self.0), token)
	}
}

use std::{marker::{PhantomData, PhantomPinned}, ops::Index};

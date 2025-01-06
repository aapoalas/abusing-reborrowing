//! Returning index newtype from methods.

fn act<'a>(
	arena: &mut Arena, token: &'a mut Token
) {														// ==>				+ 'a
    let first = arena.add(rand::random(), token); 		// ==>	+ '1		|
    let second = arena.add(rand::random(), token);		// ==>	+ '2		|
    let third = act_two(								//		|			|
        arena, first.unbind(), second.unbind(), token	// <==	- '1, '2	| &'a mut
    );													// ==>	+ '3		|
    println!("Third value: {}", arena[third]);			//		|			|
    arena.add(rand::random(), token);					//		|			|
    arena.gc(token);									// <==	- '3?		| &'a mut
    // println!("Third value: {}", arena[third]);		// 					|
														//					|
														//					|
														//					|
														//					|
														//					|
														//					|
}														// <==				-














fn act_two<'a>(
	arena: &mut Arena,
	first: ArenaIndex,
	second: ArenaIndex,
	token: &'a mut Token
) -> ArenaIndex<'a> {										// ==>				+ 'a
	let first = first.bind(token);						// ==>	+ '1		|
	let second = second.bind(token);					// ==>	+ '2		|
    println!("First value: {}", arena[first]); 			//		|			|
    println!("Second value: {}", arena[second]);		//		|			|
    arena.gc(token);									// <==	- '1, '2	|
    let third = arena.add(rand::random(), token);		// <==	  '3 == 'a	|
    third												//					|
}														// <==				-

/// Marker ZST, exclusive access to the Token is required to clean the
/// Arena.
struct Token();

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

impl Arena {
    /// Add a value to arena and return its index as VecIndex, bound to a shared
    /// borrow of Token.
    fn add<'a>(&mut self, value: u32, _: &'a Token) -> ArenaIndex<'a> {
        self.0.push(value);
        ArenaIndex(self.0.len() - 1, PhantomData)
    }

    /// Clean the arena of unwanted values, requiring exclusive access to Token.
    fn gc(&mut self, _: &mut Token) {
        self.0.retain(|_| rand::random::<bool>());
    }
}

pub(crate) fn start() {
    let mut vec = Arena(vec![0, 1, 2, 3, 4, 5]);
    let mut token = Token();
    act(&mut vec, &mut token);
}

#[derive(Clone, Copy)]
struct ArenaIndex<'a>(usize, PhantomData<&'a u32>);

impl ArenaIndex<'_> {
	/// Bind the ArenaIndex to shared a Token borrow.
    fn bind<'a>(self, _: &'a Token) -> ArenaIndex<'a> {
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

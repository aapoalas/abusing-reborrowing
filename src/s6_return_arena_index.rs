//! 6. Returning ArenaRefs from methods
//!
//! Sometimes I do want to get a result as well!

fn act<'a>(
	arena: &mut Arena, token: &'a mut Token
) {														// ==>				+ 'a
    let first = arena.get(token);				 		// ==>	+ '1		|
    let second = arena.get(token);						// ==>	+ '2		|
    let third = act_two(								//		|			|
        arena, first.unbind(), second.unbind(), token	// <==	- '1, '2	| &'a mut
    );													// ==>	+ '3		|
    println!("Third value: {}", arena[third]);			//		|			|
    arena.get(token);									//		|			|
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
	first: ArenaRef,
	second: ArenaRef,
	token: &'a mut Token
) -> ArenaRef<'a> {										// ==>				+ 'a
	let first = first.bind(token);						// ==>	+ '1		|
	let second = second.bind(token);					// ==>	+ '2		|
    println!("First value: {}", arena[first]); 			//		|			|
    println!("Second value: {}", arena[second]);		//		|			|
    arena.gc(token);									// <==	- '1, '2	|
    let third = arena.get(token);		// <==	  '3 == 'a	|
    third												//					|
}														// <==				-

/// Marker ZST, exclusive access to the Token is required to clean the
/// Arena.
struct Token();

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

impl Arena {
    /// Get an ArenaRef, bound to a shared borrow of Token.
    fn get<'a>(&mut self, _: &'a Token) -> ArenaRef<'a> {
        self.0.push(rand::random());
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

#[derive(Clone, Copy)]
struct ArenaRef<'a>(usize, PhantomData<&'a u32>);

impl ArenaRef<'_> {
	/// Bind the ArenaRef to shared a Token borrow.
    fn bind<'a>(self, _: &'a Token) -> ArenaRef<'a> {
        unsafe { std::mem::transmute::<ArenaRef, ArenaRef<'a>>(self) }
    }

	/// Forcibly release the borrow on Token.
    fn unbind(self) -> ArenaRef<'static> {
        unsafe { std::mem::transmute::<ArenaRef, ArenaRef<'static>>(self) }
    }
}

impl Index<ArenaRef<'_>> for Arena {
    type Output = u32;

    fn index(&self, index: ArenaRef<'_>) -> &Self::Output {
        self.0.index(index.0)
    }
}

use std::{marker::PhantomData, ops::Index};

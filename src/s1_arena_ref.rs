//! Step 1. Borrow values from an Arena
//!
//! Not really a garbage collector, is it?

fn act<'a>(
	arena: &'a mut Arena
) {														// ==>				+ 'a
    let first = arena.get();							// ==>	+ '1		| &'a mut
    println!("First value: {}", first);					//		|			|
    let second = arena.get();							// <=>	- '1, + '2	| &'a mut
    // println!("First value: {}", first);				//		|			|
    println!("Second value: {}", second);				//		|			|
    arena.gc();											// <==	- '2		| &'a mut
    // println!("First value: {}", first);        		//					|
    // println!("Second value: {}", second);      		//					|
    // act_two(arena);									//					|
														//					|
														//					|
														//					|
														//					|
														//					|
														//					|
}														// <==				-

impl Arena {
	/// Get a value reference from Arena.
	fn get(&mut self) -> &u32 {
	    self.0.push(rand::random());
	    self.0.last().unwrap()
	}

	/// Perform garbage collection on the Arena.
	fn gc(&mut self) {
	    self.0.retain(|_| rand::random::<bool>());
	}
}











/// Boggle the mind by using a mutable reference and a shared reference
/// reborrowed from it at the same time.
fn act_two(arena: &mut Arena) {
	// Act 2 preparation
    arena.get();
    let first = &arena[0];
    // Act 3 finale
    println!("&'a mut Arena: {:?}; &'a u32: {}", arena, first);
    arena.get();

    // Let's do that again!
    act_three(arena);
}

/// Boggle the mind by attempting to do the exact same as above, but this time
/// just move the last two lines inside a separate function.
fn act_three(arena: &mut Arena) {
	// Act 3 preparation
    arena.get();
    let first = &arena[0];
    // Act 3 finale
    // act_three_finale(arena, first);
}

/// Perfectly valid code, I guarantee it!
fn act_three_finale(arena: &mut Arena, first: &u32) {
    println!("&'a mut Arena: {:?}; &'a u32: {}", arena, first);
    arena.get();
}


















pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    act(&mut arena);
}

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

/// Make it possible to use indexing.
impl Index<usize> for Arena {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

use std::ops::Index;

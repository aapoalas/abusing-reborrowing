//! Step 2. Taking usize values from Arena
//!
//! Seems a tad error prone, doesn't it?

fn act(
	arena: &mut Arena
) {
    let first = arena.get();							// ==>	+ '1
    println!("First value: {}", arena[first]);			//		|
    let second = arena.get();							// ==>	+ '2
    println!("First value: {}", arena[first]);			//		|
    println!("Second value: {}", arena[second]);		//		|
    arena.gc();											// <==	- '1, '2
    println!("First value: {}", arena[first]);			// ?
    println!("Second value: {}", arena[second]);		// ?







}













impl Arena {
	/// Get a value reference from Arena.
	fn get(&mut self) -> usize {
		let idx = self.0.len();
	    self.0.push(rand::random());
	    idx
	}

	/// Perform garbage collection on the Arena.
	fn gc(&mut self) {
	    self.0.retain(|_| rand::random::<bool>());
	}
}

/// Make it possible to use indexing.
impl Index<usize> for Arena {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    act(&mut arena);
}

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

use std::ops::Index;

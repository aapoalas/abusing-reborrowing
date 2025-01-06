//! Taking indexes into vectors.

fn act(
	arena: &mut Arena
) {
    let first = arena.add(rand::random());				// ==>	+ '1
    println!("First value: {}", arena.get(first));		//		|
    let second = arena.add(rand::random());				// ==>	+ '2
    println!("First value: {}", arena.get(first));		//		|
    println!("Second value: {}", arena.get(second));	//		|
    arena.gc();											// <==	- '1, '2
    println!("First value: {}", arena.get(first));		// ?
    println!("Second value: {}", arena.get(second));	// ?







}













impl Arena {
	/// Add a value to Arena and return a reference to it.
	fn add(&mut self, value: u32) -> usize {
		let idx = self.0.len();
	    self.0.push(value);
	    idx
	}

	/// Perform garbage collection on the Arena.
	fn gc(&mut self) {
	    self.0.retain(|_| rand::random::<bool>());
	}

	/// Get a reference to a value in the Arena.
	fn get(&self, idx: usize) -> &u32 {
		&self.0[idx]
	}
}

pub(crate) fn start() {
    let mut arena = Arena(vec![0, 1, 2, 3, 4, 5]);
    act(&mut arena);
}

/// Garbage collected heap arena.
#[derive(Debug, Clone)]
struct Arena(Vec<u32>);

//! Index newtype into vectors.

#[allow(unused)]
fn act(vec: &mut Vec<u32>) {
    let first_index = add(vec, rand::random());			// <--	+ '1
    // println!("First value: {}", vec[first_index]);	//		|
    let second_index = add(vec, rand::random());		// <--	+ '2
    // println!("First value: {}", vec[first_index]);	//		|
    // println!("Second value: {}", vec[second_index]);	//		|
    clean(vec);											//		- '1, '2
    // println!("First value: {}", vec[first_index]);	//		?
    // println!("Second value: {}", vec[second_index]);	//		?
}

/// Index into a Vec<u32> with a lifetime.
#[derive(Clone, Copy)]
struct VecIndex<'a>(usize, PhantomData<&'a u32>);

/// Make it possible to use indexing.
impl Index<VecIndex<'_>> for Vec<u32> {
    type Output = u32;

    fn index(&self, index: VecIndex<'_>) -> &Self::Output {
        self.index(index.0)
    }
}

/// Add a value to vector and return its index as VecIndex.
fn add(vec: &mut Vec<u32>, value: u32) -> VecIndex {
    vec.push(value);
    VecIndex(vec.len() - 1, PhantomData)
}









impl VecIndex<'_> {
    /// Forcibly release the borrow on Vec<u32>.
    #[allow(dead_code)]
    fn unbind(self) -> VecIndex<'static> {
        unsafe { std::mem::transmute::<VecIndex, VecIndex<'static>>(self) }
    }
}

fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}

use std::{marker::PhantomData, ops::Index};

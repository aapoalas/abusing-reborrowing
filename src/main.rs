#![allow(dead_code, unused, clippy::needless_lifetimes)]

mod s1_vec;
mod s2_vec_index;
mod s3_arena_index;
mod s4_arena_index_with_token;
mod s5_pass_arena_index;
mod s6_return_arena_index;
mod s7_zst_token;
mod s8_scoped_rooting;

fn main() {
    s1_vec::start();
    s2_vec_index::start();
    s3_arena_index::start();
    s4_arena_index_with_token::start();
    s5_pass_arena_index::start();
    s6_return_arena_index::start();
    s7_zst_token::start();
    s8_scoped_rooting::start();
}

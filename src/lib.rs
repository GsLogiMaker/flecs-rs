#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Allow some bindgen warnings for now
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

use std::{any::TypeId, mem::MaybeUninit};

pub use flecs_sys::*;

////////////////////////////////////////////////////////////////////////////////////////////////////////
// This Rust binding for flecs is a WIP!!!
//
// Possible TODOs:
// - [ ] audit & fix up ALL string usages. rust -> C must null terminate!
// - [ ] change all get<> component funcs to return Option<>?
// - [ ] validate that term components were named earlier in chain?
// - [ ] We can only safely store primitives and raw pointer types within
//		components currently, due to how the raw memory is inserted/moved
//		need to look in to hooking the lifecycle support to rust, etc
//		This could become a bit of a deal breaker for idiomatic rust
// 		component storage if not solved
// - [ ] Implement proper Rusty Query / System APIs that use Tuple generics

////////////////////////////////////////////////////////////////////////////////////////////////////////

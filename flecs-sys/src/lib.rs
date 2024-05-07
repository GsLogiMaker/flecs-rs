#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Allow some bindgen warnings for now
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

use std::{ffi::c_void, mem::MaybeUninit};

pub mod bindings;
pub use bindings::*;

// C Struct initializer Defaults
//
impl Default for ecs_component_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_entity_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_filter_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_observer_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_query_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_system_desc_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl Default for ecs_term_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

impl ecs_type_hooks_t {
	fn set_binding_ctx<T>(&mut self, data: T) {
		let ptr:*mut T = Box::leak(Box::new(data));
		self.binding_ctx = ptr.cast::<c_void>()
	}
} impl Default for ecs_type_hooks_t {
	fn default() -> Self {
		unsafe { MaybeUninit::zeroed().assume_init() }
	}
}

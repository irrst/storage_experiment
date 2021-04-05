use std::{alloc::Layout, mem};

pub fn layout_for<T>(repeat: usize) -> Layout {
    unsafe { Layout::from_size_align_unchecked(mem::size_of::<T>() * repeat, mem::align_of::<T>()) }
}

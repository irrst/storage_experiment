use std::{
    alloc::Layout,
    cell::UnsafeCell,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::NonNull,
};

use crate::traits::{AllocError, Storage};

pub struct InlineHandleStorage<ALIGN, const SIZE: usize> {
    _marker: PhantomData<ALIGN>,
}

pub struct InlineHandle<ALIGN, const SIZE: usize> {
    inline: UnsafeCell<[u8; SIZE]>,
    _align: [ALIGN; 0],
}

impl<ALIGN, const SIZE: usize> Storage for InlineHandleStorage<ALIGN, SIZE> {
    type Handle = InlineHandle<ALIGN, SIZE>;
    type Context = ();

    unsafe fn allocate(layout: Layout, _: &mut Self::Context) -> Result<Self::Handle, AllocError> {
        if layout.size() <= mem::size_of::<ALIGN>() && layout.align() <= mem::align_of::<ALIGN>() {
            Ok(Self::Handle {
                inline: UnsafeCell::new(MaybeUninit::zeroed().assume_init()),
                _align: [],
            })
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(_: Layout, _: &Self::Handle, _: &mut Self::Context) {
        // this function call will NOT deallocate any memory
    }

    unsafe fn as_ptr(_: Layout, handle: &Self::Handle, _: &Self::Context) -> NonNull<u8> {
        NonNull::new_unchecked(handle.inline.get().cast::<u8>())
    }
}

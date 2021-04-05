use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr::NonNull,
};

use crate::traits::{AllocError, Storage};

pub struct InlineBumpStorage<S> {
    _marker: PhantomData<S>,
}

pub struct InlineBumpContext<S> {
    storage: UnsafeCell<MaybeUninit<S>>,
    tracker: usize,
}

pub struct InlineBumpHandle {
    at: usize,
}

impl<S> Storage for InlineBumpStorage<S> {
    type Handle = InlineBumpHandle;
    type Context = InlineBumpContext<S>;

    unsafe fn allocate(
        layout: std::alloc::Layout,
        context: &mut Self::Context,
    ) -> Result<Self::Handle, AllocError> {
        if layout.align() > mem::align_of::<S>() {
            Err(AllocError)
        } else {
            match context.tracker % layout.align() {
                0 => context.tracker += layout.size(),
                other => context.tracker += layout.size() + layout.align() - other,
            };
            if context.tracker > mem::size_of::<S>() {
                Err(AllocError)
            } else {
                Ok(InlineBumpHandle {
                    at: context.tracker - layout.size(),
                })
            }
        }
    }

    unsafe fn deallocate(_: std::alloc::Layout, _: &Self::Handle, _: &mut Self::Context) {
        // this function will NOT deallocate any memory
    }

    unsafe fn as_ptr(
        _: std::alloc::Layout,
        handle: &Self::Handle,
        context: &Self::Context,
    ) -> NonNull<u8> {
        NonNull::new_unchecked(context.storage.get().cast::<u8>().add(handle.at))
    }
}

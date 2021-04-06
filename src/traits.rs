use std::{
    alloc::Layout,
    ptr::{self, NonNull},
};

pub trait Storage
where
    Self: Sized,
{
    type Handle;
    type Context;

    unsafe fn allocate(
        layout: Layout,
        context: &mut Self::Context,
    ) -> Result<Self::Handle, AllocError>;

    unsafe fn deallocate(layout: Layout, handle: &Self::Handle, context: &mut Self::Context);

    unsafe fn as_ptr(layout: Layout, handle: &Self::Handle, context: &Self::Context)
        -> NonNull<u8>;

    unsafe fn reallocate(
        old_layout: Layout,
        new_layout: Layout,
        old_handle: &Self::Handle,
        context: &mut Self::Context,
    ) -> Result<Self::Handle, AllocError> {
        let new_handle = Self::allocate(new_layout, context)?;
        let size = if old_layout.size() < new_layout.size() {
            old_layout.size()
        } else {
            new_layout.size()
        };
        ptr::copy_nonoverlapping(
            Self::as_ptr(old_layout, old_handle, context).as_ptr(),
            Self::as_ptr(new_layout, &new_handle, context).as_ptr(),
            size,
        );
        Self::deallocate(old_layout, old_handle, context);
        Ok(new_handle)
    }
}

pub struct AllocError;

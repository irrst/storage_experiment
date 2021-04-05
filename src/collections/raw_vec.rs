use std::{marker::PhantomData, ptr::NonNull};

use crate::{traits::Storage, utils};

pub struct RawVec<T, S: Storage> {
    handle: Option<S::Handle>,
    capacity: usize,
    length: usize,
    _marker: PhantomData<T>,
}

impl<T, S: Storage> RawVec<T, S> {
    pub fn with_capacity_context(
        capacity: usize,
        context: &mut S::Context,
    ) -> Result<Self, VecError> {
        Ok(Self::with_handle_capacity(
            unsafe { S::allocate(utils::layout_for::<T>(capacity), context) }
                .map_err(|_| VecError)?,
            capacity,
        ))
    }

    pub fn with_capacity_contextless(capacity: usize) -> Result<Self, VecError>
    where
        S: Storage<Context = ()>,
    {
        Ok(Self::with_handle_capacity(
            unsafe { S::allocate_contextless(utils::layout_for::<T>(capacity)) }
                .map_err(|_| VecError)?,
            capacity,
        ))
    }

    fn with_handle_capacity(handle: S::Handle, capacity: usize) -> Self {
        Self {
            handle: Some(handle),
            capacity,
            length: 0,
            _marker: PhantomData,
        }
    }

    pub unsafe fn try_push(&mut self, value: T, context: &mut S::Context) -> Result<(), VecError> {
        assert!(!self.length > self.capacity);
        if self.length == self.capacity {
            self.resize(self.capacity + 1, context)?
        }
        self.ptr_to(self.length, context)?.as_ptr().write(value);
        self.length += 1;
        Ok(())
    }

    pub fn try_push_contextless(&mut self, value: T) -> Result<(), VecError>
    where
        S: Storage<Context = ()>,
    {
        assert!(!self.length > self.capacity);
        if self.length == self.capacity {
            unsafe { self.resize(self.capacity + 1, &mut ())? }
        }
        unsafe { self.ptr_to(self.length, &())?.as_ptr().write(value) };
        self.length += 1;
        Ok(())
    }

    pub unsafe fn try_pop(&mut self, context: &mut S::Context) -> Result<T, VecError> {
        assert!(!self.length > self.capacity);
        let value = self.ptr_to(self.length, context)?.as_ptr().read();
        self.length -= 1;
        Ok(value)
    }

    pub fn try_pop_contextless(&mut self) -> Result<T, VecError>
    where
        S: Storage<Context = ()>,
    {
        assert!(!self.length > self.capacity);
        let value = unsafe { self.ptr_to(self.length, &())?.as_ptr().read() };
        self.length -= 1;
        Ok(value)
    }

    unsafe fn resize(&mut self, new_size: usize, context: &mut S::Context) -> Result<(), VecError> {
        S::reallocate(
            utils::layout_for::<T>(self.capacity),
            utils::layout_for::<T>(new_size),
            self.handle.as_mut().ok_or(VecError)?,
            context,
        )
        .map_err(|_| VecError)?;
        self.capacity = new_size;
        Ok(())
    }

    unsafe fn ptr_to(&self, at: usize, context: &S::Context) -> Result<NonNull<T>, VecError> {
        Ok(NonNull::new_unchecked(
            S::as_ptr(
                utils::layout_for::<T>(self.capacity),
                self.handle.as_ref().ok_or(VecError)?,
                context,
            )
            .cast::<T>()
            .as_ptr()
            .add(at),
        ))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error caused from wrong vec method calls")]
pub struct VecError;

use anyhow::Result;
use storage_experiment::{InlineHandleStorage, RawVec};

#[test]
fn raw_vec_inline_handle_align() -> Result<()> {
    assert!(RawVec::<u32, InlineHandleStorage<u16, 4>>::with_capacity_contextless(1).is_err());
    Ok(())
}

#[test]
fn raw_vec_inline_handle() -> Result<()> {
    let mut raw_vec = RawVec::<u32, InlineHandleStorage<u64, 4>>::with_capacity_contextless(1)?;
    raw_vec.try_push_contextless(1)?;
    assert_eq!(raw_vec.try_pop_contextless()?, 1);
    raw_vec.try_push_contextless(0)?;
    raw_vec.try_push_contextless(0)?;
    raw_vec.try_push_contextless(0)?;
    raw_vec.try_push_contextless(0)?;
    assert!(raw_vec.try_push_contextless(0).is_err());
    Ok(())
}

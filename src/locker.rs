use crate::Isolate;
use crate::isolate::RealIsolate;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::Rc;

unsafe extern "C" {
  fn v8__Locker__CONSTRUCT(
    buf: *mut MaybeUninit<Locker>,
    isolate: *mut RealIsolate,
  );
  fn v8__Locker__DESTRUCT(this: *mut Locker);
}

/// Acquires V8's per-isolate lock on the current thread.
///
/// Hold this guard for the full duration of any cross-thread isolate entry and
/// use. It is the Rust wrapper around `v8::Locker`.
#[repr(C)]
pub struct Locker([usize; 2], PhantomData<Rc<()>>);

impl Locker {
  #[inline(always)]
  pub fn new(isolate: &Isolate) -> Self {
    let mut buf = MaybeUninit::<Self>::uninit();
    unsafe {
      v8__Locker__CONSTRUCT(&mut buf, isolate.as_real_ptr());
      buf.assume_init()
    }
  }
}

impl Drop for Locker {
  #[inline(always)]
  fn drop(&mut self) {
    unsafe { v8__Locker__DESTRUCT(self) }
  }
}

use crate::Global;
use crate::OwnedIsolate;

/// Explicitly sendable wrapper for moving an exited isolate between threads.
///
/// # Safety
///
/// The caller must ensure the isolate is exited before wrapping it, is not
/// concurrently used from multiple threads, and that any cross-thread re-entry
/// follows V8's locking contract.
#[derive(Debug)]
pub struct SendableOwnedIsolate(OwnedIsolate);

unsafe impl Send for SendableOwnedIsolate {}

impl SendableOwnedIsolate {
  /// Wraps an exited isolate so it can be moved to another thread.
  pub unsafe fn new(isolate: OwnedIsolate) -> Self {
    Self(isolate)
  }

  /// Returns the wrapped isolate. Before using or disposing it on the current
  /// thread, reacquire V8's lock and enter the isolate again.
  pub fn into_inner(self) -> OwnedIsolate {
    self.0
  }
}

/// Explicitly sendable wrapper for moving persistent handles between threads.
///
/// # Safety
///
/// The caller must ensure the wrapped handle is only used with the isolate it
/// belongs to and under the same cross-thread synchronization discipline as the
/// isolate itself, including any required `Locker` usage around re-entry.
#[derive(Debug)]
pub struct SendableGlobal<T>(Global<T>);

unsafe impl<T> Send for SendableGlobal<T> {}

impl<T> SendableGlobal<T> {
  pub unsafe fn new(global: Global<T>) -> Self {
    Self(global)
  }

  pub fn into_inner(self) -> Global<T> {
    self.0
  }
}

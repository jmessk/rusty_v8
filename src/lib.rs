// Copyright 2019-2021 the Deno authors. All rights reserved. MIT license.

//! # Example
//!
//! ```rust
//! let platform = v8::new_default_platform(0, false).make_shared();
//! v8::V8::initialize_platform(platform);
//! v8::V8::initialize();
//!
//! let isolate = &mut v8::Isolate::new(Default::default());
//!
//! let scope = std::pin::pin!(v8::HandleScope::new(isolate));
//! let scope = &mut scope.init();
//! let context = v8::Context::new(scope, Default::default());
//! let scope = &mut v8::ContextScope::new(scope, context);
//!
//! let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
//! println!("javascript code: {}", code.to_rust_string_lossy(scope));
//!
//! let script = v8::Script::compile(scope, code, None).unwrap();
//! let result = script.run(scope).unwrap();
//! let result = result.to_string(scope).unwrap();
//! println!("result: {}", result.to_rust_string_lossy(scope));
//! ```

#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate bitflags;
extern crate temporal_capi;

mod array_buffer;
mod array_buffer_view;
mod bigint;
mod binding;
mod context;
pub use context::ContextOptions;
pub mod cppgc;
mod data;
mod date;
mod exception;
mod external;
mod external_references;
pub mod fast_api;
mod fixed_array;
mod function;
mod gc;
mod get_property_names_args_builder;
mod handle;
pub mod icu;
mod isolate;
mod isolate_create_params;
mod locker;
mod microtask;
mod module;
mod name;
mod number;
mod object;
mod platform;
mod primitive_array;
mod primitives;
mod private;
mod promise;
mod property_attribute;
mod property_descriptor;
mod property_filter;
mod property_handler_flags;
mod proxy;
mod regexp;
mod scope;
mod script;
mod script_or_module;
mod sendable;
mod shared_array_buffer;
mod snapshot;
mod string;
mod support;
mod symbol;
mod template;
mod typed_array;
mod unbound_module_script;
mod unbound_script;
mod value;
mod value_deserializer;
mod value_serializer;
mod wasm;

pub mod crdtp;
pub mod inspector;
pub mod json;
pub mod script_compiler;
#[cfg(feature = "simdutf")]
pub mod simdutf;
// This module is intentionally named "V8" rather than "v8" to match the
// C++ namespace "v8::V8".
#[allow(non_snake_case)]
pub mod V8;

pub use array_buffer::*;
pub use data::*;
pub use exception::*;
pub use external_references::ExternalReference;
pub use function::*;
pub use gc::*;
pub use get_property_names_args_builder::*;
pub use handle::Eternal;
pub use handle::Global;
pub use handle::Handle;
pub use handle::Local;
pub use handle::SealedLocal;
pub use handle::TracedReference;
pub use handle::Weak;
pub use isolate::GarbageCollectionType;
pub use isolate::HeapCodeStatistics;
pub use isolate::HeapSpaceStatistics;
pub use isolate::HeapStatistics;
pub use isolate::HostCreateShadowRealmContextCallback;
pub use isolate::HostImportModuleDynamicallyCallback;
pub use isolate::HostImportModuleWithPhaseDynamicallyCallback;
pub use isolate::HostInitializeImportMetaObjectCallback;
pub use isolate::Isolate;
pub use isolate::IsolateHandle;
pub use isolate::MemoryPressureLevel;
pub use isolate::MessageCallback;
pub use isolate::MessageErrorLevel;
pub use isolate::MicrotasksPolicy;
pub use isolate::ModuleImportPhase;
pub use isolate::NearHeapLimitCallback;
pub use isolate::OomDetails;
pub use isolate::OomErrorCallback;
pub use isolate::OwnedIsolate;
pub use isolate::PromiseHook;
pub use isolate::PromiseHookType;
pub use isolate::PromiseRejectCallback;
pub use isolate::RealIsolate;
pub use isolate::TimeZoneDetection;
pub use isolate::UseCounterCallback;
pub use isolate::UseCounterFeature;
pub use isolate::WasmAsyncSuccess;
pub use isolate_create_params::CreateParams;
pub use microtask::MicrotaskQueue;
pub use module::*;
pub use object::*;
pub use platform::IdleTask;
pub use platform::Platform;
pub use platform::PlatformImpl;
pub use platform::Task;
pub use platform::new_custom_platform;
pub use platform::new_default_platform;
pub use platform::new_single_threaded_default_platform;
pub use platform::new_unprotected_default_platform;
pub use primitives::*;
pub use promise::{PromiseRejectEvent, PromiseRejectMessage, PromiseState};
pub use property_attribute::*;
pub use property_descriptor::*;
pub use property_filter::*;
pub use property_handler_flags::*;
pub use regexp::RegExpCreationFlags;
pub use scope::AllowJavascriptExecutionScope;
// pub use scope::CallbackScope;
pub use scope::CallbackScope;
pub use scope::ContextScope;
pub use scope::DisallowJavascriptExecutionScope;
pub use scope::EscapableHandleScope;
pub use scope::PinCallbackScope;
pub use scope::PinScope;
pub use scope::PinnedRef;
pub use scope::ScopeStorage;
// pub use scope::HandleScope;
pub use isolate::UnsafeRawIsolatePtr;
pub use locker::Locker;
pub use scope::HandleScope;
pub use scope::OnFailure;
pub use scope::TryCatch;
pub use script::ScriptOrigin;
pub use script_compiler::CachedData;
pub use sendable::SendableGlobal;
pub use sendable::SendableOwnedIsolate;
pub use snapshot::FunctionCodeHandling;
pub use snapshot::StartupData;
pub use string::Encoding;
pub use string::NewStringType;
pub use string::OneByteConst;
pub use string::ValueView;
pub use string::ValueViewData;
pub use string::WriteFlags;
pub use string::WriteOptions;
pub use string::latin1_to_utf8;
pub use support::SharedPtr;
pub use support::SharedRef;
pub use support::UniquePtr;
pub use support::UniqueRef;
pub use template::*;
pub use value_deserializer::ValueDeserializer;
pub use value_deserializer::ValueDeserializerHelper;
pub use value_deserializer::ValueDeserializerImpl;
pub use value_serializer::ValueSerializer;
pub use value_serializer::ValueSerializerHelper;
pub use value_serializer::ValueSerializerImpl;
pub use wasm::CompiledWasmModule;
pub use wasm::ModuleCachingInterface;
pub use wasm::WasmModuleCompilation;
pub use wasm::WasmStreaming;

/// https://v8.dev/docs/version-numbers
pub const MAJOR_VERSION: u32 = binding::v8__MAJOR_VERSION;
/// https://v8.dev/docs/version-numbers
pub const MINOR_VERSION: u32 = binding::v8__MINOR_VERSION;
/// https://v8.dev/docs/version-numbers
pub const BUILD_NUMBER: u32 = binding::v8__BUILD_NUMBER;
/// https://v8.dev/docs/version-numbers
pub const PATCH_LEVEL: u32 = binding::v8__PATCH_LEVEL;
/// https://v8.dev/docs/version-numbers
pub const VERSION_STRING: &str =
  // TODO: cleanup when Result::unwrap is const stable.
  match binding::v8__VERSION_STRING.to_str() {
    Ok(v) => v,
    Err(_) => panic!("Unable to convert CStr to &str??"),
  };

// TODO(piscisaureus): Ideally this trait would not be exported.
pub use support::MapFnTo;

pub const TYPED_ARRAY_MAX_SIZE_IN_HEAP: usize =
  binding::v8__TYPED_ARRAY_MAX_SIZE_IN_HEAP as _;

#[cfg(test)]
#[allow(unused)]
pub(crate) fn initialize_v8() {
  use std::sync::Once;

  static INIT: Once = Once::new();
  INIT.call_once(|| {
    V8::initialize_platform(new_default_platform(0, false).make_shared());
    V8::initialize();
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Once;

  fn initialize_v8_unprotected() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
      V8::initialize_platform(
        new_unprotected_default_platform(0, false).make_shared(),
      );
      V8::initialize();
    });
  }

  fn increment_counter(
    isolate: &mut OwnedIsolate,
    context: &Global<Context>,
  ) -> i32 {
    let scope = std::pin::pin!(HandleScope::new(isolate));
    let scope = &mut scope.init();
    let context = Local::new(scope, context);
    let scope = &mut ContextScope::new(scope, context);
    let source = String::new(
      scope,
      "globalThis.counter = (globalThis.counter ?? 0) + 1; globalThis.counter",
    )
    .unwrap();
    let script = Script::compile(scope, source, None).unwrap();
    let value = script.run(scope).unwrap();
    value.int32_value(scope).unwrap()
  }

  fn run_fresh_context_script(isolate: &mut OwnedIsolate, source: &str) -> i32 {
    let scope = std::pin::pin!(HandleScope::new(isolate));
    let scope = &mut scope.init();
    let context = Context::new(scope, Default::default());
    let scope = &mut ContextScope::new(scope, context);
    let source = String::new(scope, source).unwrap();
    let script = Script::compile(scope, source, None).unwrap();
    let value = script.run(scope).unwrap();
    value.int32_value(scope).unwrap()
  }

  #[cfg(not(target_os = "android"))]
  #[test]
  fn isolate_can_call_js_with_locker_same_thread() {
    initialize_v8();

    let mut isolate = Isolate::new_sendable(Default::default()).into_inner();
    let _locker = Locker::new(&isolate);
    unsafe { isolate.enter() };
    assert_eq!(run_fresh_context_script(&mut isolate, "1 + 1"), 2);
  }

  #[cfg(not(target_os = "android"))]
  #[ignore = "documents the current failure mode without Locker"]
  #[test]
  fn isolate_cannot_migrate_threads_with_fresh_contexts_without_locker() {
    initialize_v8_unprotected();

    let mut isolate = Isolate::new(Default::default());
    assert_eq!(run_fresh_context_script(&mut isolate, "1 + 1"), 2);
    unsafe { isolate.exit() };

    let isolate = unsafe { SendableOwnedIsolate::new(isolate) };
    let handle = std::thread::spawn(move || {
      let mut isolate = isolate.into_inner();
      unsafe { isolate.enter() };
      let value = run_fresh_context_script(&mut isolate, "2 + 2");
      unsafe { isolate.exit() };
      (unsafe { SendableOwnedIsolate::new(isolate) }, value)
    });

    let (isolate, value) = handle.join().unwrap();
    assert_eq!(value, 4);

    let mut isolate = isolate.into_inner();
    unsafe { isolate.enter() };
    assert_eq!(run_fresh_context_script(&mut isolate, "3 + 3"), 6);
    drop(isolate);
  }

  #[cfg(not(target_os = "android"))]
  #[ignore = "documents the current failure mode without Locker"]
  #[test]
  fn isolate_cannot_migrate_threads_and_call_js_without_locker() {
    initialize_v8_unprotected();

    let mut isolate = Isolate::new(Default::default());
    let context = {
      let scope = std::pin::pin!(HandleScope::new(&mut isolate));
      let scope = &mut scope.init();
      let context = Context::new(scope, Default::default());
      Global::new(scope, context)
    };

    let mut current = increment_counter(&mut isolate, &context);
    assert_eq!(current, 1);
    unsafe { isolate.exit() };

    let mut isolate = unsafe { SendableOwnedIsolate::new(isolate) };
    let mut context = unsafe { SendableGlobal::new(context) };

    for _ in 0..10 {
      let handle = std::thread::spawn(move || {
        let mut isolate = isolate.into_inner();
        let context = context.into_inner();
        unsafe { isolate.enter() };
        let current = increment_counter(&mut isolate, &context);
        unsafe { isolate.exit() };
        (
          unsafe { SendableOwnedIsolate::new(isolate) },
          unsafe { SendableGlobal::new(context) },
          current,
        )
      });

      let (returned_isolate, returned_context, worker_current) =
        handle.join().unwrap();
      isolate = returned_isolate;
      context = returned_context;

      current += 1;
      assert_eq!(worker_current, current);

      let mut main_isolate = isolate.into_inner();
      let main_context = context.into_inner();
      unsafe { main_isolate.enter() };
      current = increment_counter(&mut main_isolate, &main_context);
      unsafe { main_isolate.exit() };
      isolate = unsafe { SendableOwnedIsolate::new(main_isolate) };
      context = unsafe { SendableGlobal::new(main_context) };
    }

    let mut isolate = isolate.into_inner();
    let context = context.into_inner();
    unsafe { isolate.enter() };
    assert_eq!(increment_counter(&mut isolate, &context), current + 1);
    drop(context);
    drop(isolate);
  }

  #[cfg(not(target_os = "android"))]
  #[test]
  fn isolate_can_migrate_threads_with_fresh_contexts_and_locker() {
    initialize_v8();

    let mut isolate = Isolate::new_sendable(Default::default()).into_inner();
    let locker = Locker::new(&isolate);
    unsafe { isolate.enter() };
    assert_eq!(run_fresh_context_script(&mut isolate, "1 + 1"), 2);
    unsafe { isolate.exit() };
    drop(locker);

    let isolate = unsafe { SendableOwnedIsolate::new(isolate) };
    let handle = std::thread::spawn(move || {
      let mut isolate = isolate.into_inner();
      let locker = Locker::new(&isolate);
      unsafe { isolate.enter() };
      let value = run_fresh_context_script(&mut isolate, "2 + 2");
      unsafe { isolate.exit() };
      drop(locker);
      (unsafe { SendableOwnedIsolate::new(isolate) }, value)
    });

    let (isolate, value) = handle.join().unwrap();
    assert_eq!(value, 4);
    let mut isolate = isolate.into_inner();
    let _locker = Locker::new(&isolate);
    unsafe { isolate.enter() };
    assert_eq!(run_fresh_context_script(&mut isolate, "3 + 3"), 6);
  }

  #[cfg(not(target_os = "android"))]
  #[test]
  fn isolate_can_migrate_threads_and_call_js_with_persistent_context_and_locker()
   {
    initialize_v8();

    let mut isolate = Isolate::new_sendable(Default::default()).into_inner();
    let locker = Locker::new(&isolate);
    unsafe { isolate.enter() };
    let context = {
      let scope = std::pin::pin!(HandleScope::new(&mut isolate));
      let scope = &mut scope.init();
      let context = Context::new(scope, Default::default());
      Global::new(scope, context)
    };

    let mut current = increment_counter(&mut isolate, &context);
    assert_eq!(current, 1);
    unsafe { isolate.exit() };
    drop(locker);

    let mut isolate = unsafe { SendableOwnedIsolate::new(isolate) };
    let mut context = unsafe { SendableGlobal::new(context) };

    for _ in 0..10 {
      let handle = std::thread::spawn(move || {
        let mut isolate = isolate.into_inner();
        let context = context.into_inner();
        let locker = Locker::new(&isolate);
        unsafe { isolate.enter() };
        let current = increment_counter(&mut isolate, &context);
        unsafe { isolate.exit() };
        drop(locker);
        (
          unsafe { SendableOwnedIsolate::new(isolate) },
          unsafe { SendableGlobal::new(context) },
          current,
        )
      });

      let (returned_isolate, returned_context, worker_current) =
        handle.join().unwrap();
      isolate = returned_isolate;
      context = returned_context;

      current += 1;
      assert_eq!(worker_current, current);

      let mut main_isolate = isolate.into_inner();
      let main_context = context.into_inner();
      let locker = Locker::new(&main_isolate);
      unsafe { main_isolate.enter() };
      current = increment_counter(&mut main_isolate, &main_context);
      unsafe { main_isolate.exit() };
      drop(locker);
      isolate = unsafe { SendableOwnedIsolate::new(main_isolate) };
      context = unsafe { SendableGlobal::new(main_context) };
    }
    let mut isolate = isolate.into_inner();
    let context = context.into_inner();
    let _locker = Locker::new(&isolate);
    unsafe { isolate.enter() };
    assert_eq!(increment_counter(&mut isolate, &context), current + 1);
  }
}

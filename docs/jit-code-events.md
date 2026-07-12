# JIT Code Events in `rusty_v8`

`rusty_v8` now exposes V8's `JitCodeEventHandler` through
`CreateParams::jit_code_event_handler`.

This callback is useful when you want to count JIT activity or record when JIT
code is emitted, without collecting compile durations.

## Exposed API

- `v8::CreateParams::jit_code_event_handler`
- `v8::JitCodeEvent`
- `v8::JitCodeEventType`
- `v8::JitCodeCodeType`

`JitCodeEvent` currently exposes:

- `event_type()`
- `code_type()`
- `code_start()`
- `code_len()`

## Counting JIT code additions

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static JIT_CODE_ADDED: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn on_jit_code_event(event: &v8::JitCodeEvent) {
  if event.event_type() == v8::JitCodeEventType::CodeAdded
    && event.code_type() == v8::JitCodeCodeType::JitCode
  {
    JIT_CODE_ADDED.fetch_add(1, Ordering::Relaxed);
  }
}

let params = v8::CreateParams::default().jit_code_event_handler(on_jit_code_event);
let isolate = &mut v8::Isolate::new(params);
```

If you also want timing information, record a host timestamp in the callback.

## Notes

- `CodeAdded + JitCode` is the most useful combination for counting generated
  JIT code.
- `CodeMoved` is usually a relocation event, not a fresh compile.
- V8 documents that `CodeRemoved` events are not currently issued.
- This API does not reliably distinguish Sparkplug, Maglev, and TurboFan.
  It is best suited to aggregate JIT activity.

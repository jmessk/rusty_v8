#include <new>

#include "v8-locker.h"

extern "C" {

void v8__Locker__CONSTRUCT(void* buf, v8::Isolate* isolate) {
  new (buf) v8::Locker(isolate);
}

void v8__Locker__DESTRUCT(v8::Locker* self) { self->~Locker(); }

}

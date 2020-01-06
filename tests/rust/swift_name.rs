use std::sync::Arc;

struct Foo {
  bar: i32,
}

#[repr(C)]
struct FooRef {
  ptr: *mut Foo
}

impl FooRef {
  #[export_name="FooRef_create"]
  pub extern fn create() -> FooRef {
    Box::into_raw(Box::new(Foo {
      bar: 0
    }))
  }

  #[export_name="FooRef_setBar"]
  pub extern fn set_bar(self: FooRef, bar: i32) {
    if let Some(nonnull) = std::ptr::NonNull::new(self.ptr) {
      nonnull.as_mut().bar = bar;
    }
  }

  #[export_name="FooRef_getBar"]
  pub extern fn get_bar(self: FooRef) -> i32 {
    if let Some(nonnull) = std::ptr::NonNull::new(self.ptr) {
      nonnull.as_ref().bar
    }
  }

  /// cbindgen:postfix=/*a comment!*/
  #[export_name="FooRef_doThing"]
  pub extern fn do_thing(self: FooRef) -> i32 {
    if let Some(nonnull) = std::ptr::NonNull::new(self.ptr) {
      nonnull.as_ref().bar
    }
  }
}

#[no_mangle]
pub extern fn do_the_thing() {

}

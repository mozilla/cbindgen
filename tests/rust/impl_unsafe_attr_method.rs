#[repr(C)]
pub struct DummyStruct {
    dummy_field: i32,
}


impl DummyStruct {
    #[unsafe(export_name = "new_dummy")]
    pub const extern "C" fn new() -> Self {
        Self {
            dummy_field: 0,
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn new_dummy_param(dummy_field: i32) -> Self {
        Self {
            dummy_field,
        }
    }
}

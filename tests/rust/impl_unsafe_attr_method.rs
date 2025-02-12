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
}

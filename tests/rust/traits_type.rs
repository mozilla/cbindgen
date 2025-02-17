pub trait DummyTrait {
    type DummyIn;
    type DummyOut;

    extern "C" fn dummy(self, in_: Self::DummyIn) -> Self::DummyOut;
}

#[repr(C)]
pub struct Dummy0 {
    dummy: usize,
}

impl DummyTrait for Dummy0 {
    type DummyIn = usize;
    type DummyOut = Self;

    #[unsafe(export_name = "dummy_Dummy0")]
    extern "C" fn dummy(self, in_: Self::DummyIn) -> Self::DummyOut {
        Self {
            dummy: in_,
        }
    }
}

#[repr(C)]
pub struct Dummy1 {
    dummy: usize
}

impl DummyTrait for Dummy1 {
    type DummyIn = ();
    type DummyOut = i32;

    #[unsafe(export_name = "dummy_Dummy1")]
    extern "C" fn dummy(self, in_: Self::DummyIn) -> Self::DummyOut {
        0
    }
}

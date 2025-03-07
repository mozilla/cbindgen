pub trait DummyTrait {
    type DummyIn0;
    type DummyIn1;
    type DummyOut;

    extern "C" fn dummy(self, in0: Self::DummyIn0, in1: Self::DummyIn1) -> Self::DummyOut;
}

#[repr(C)]
pub struct Dummy0 {
    dummy: usize,
}

impl DummyTrait for Dummy0 {
    type DummyIn0 = ();
    type DummyIn1 = Self;
    type DummyOut = Self;

    #[unsafe(export_name = "dummy_Dummy0")]
    extern "C" fn dummy(self, in0: Self::DummyIn0, _in1: Self::DummyIn1) -> Self::DummyOut {
        Self {
            dummy: in0,
        }
    }
}

#[repr(C)]
pub struct Dummy1 {
    dummy: usize
}

impl DummyTrait for Dummy1 {
    type DummyIn0 = ();
    type DummyIn1 = usize;
    type DummyOut = i32;

    #[unsafe(export_name = "dummy_Dummy1")]
    extern "C" fn dummy(self, _in0: Self::DummyIn0, _in1: Self::DummyIn1) -> Self::DummyOut {
        0
    }
}

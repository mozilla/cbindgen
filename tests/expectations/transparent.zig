const std = @import("std");

const DummyStruct = opaque {};

const EnumWithAssociatedConstantInImpl = opaque {};

pub const TransparentComplexWrappingStructTuple = DummyStruct;

pub const TransparentPrimitiveWrappingStructTuple = u32;

pub const TransparentComplexWrappingStructure = DummyStruct;

pub const TransparentPrimitiveWrappingStructure = u32;

pub const TransparentComplexWrapper_i32 = DummyStruct;

pub const TransparentPrimitiveWrapper_i32 = u32;

pub const TransparentPrimitiveWithAssociatedConstants = u32;
pub const TransparentPrimitiveWithAssociatedConstants_ZERO = 0;
pub const TransparentPrimitiveWithAssociatedConstants_ONE = 1;

pub const EnumWithAssociatedConstantInImpl_TEN = 10;

extern fn root(a: TransparentComplexWrappingStructTuple, b: TransparentPrimitiveWrappingStructTuple, c: TransparentComplexWrappingStructure, d: TransparentPrimitiveWrappingStructure, e: TransparentComplexWrapper_i32, f: TransparentPrimitiveWrapper_i32, g: TransparentPrimitiveWithAssociatedConstants, h: EnumWithAssociatedConstantInImpl) anyopaque;

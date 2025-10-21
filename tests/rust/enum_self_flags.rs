/// A three-bit value that represents the axis in which position-area operates on.
/// Represented as 3 bits: axis type (physical or logical), direction type (physical or logical),
/// axis value.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PositionAreaAxis {
    Horizontal = 0b000,
    Vertical = 0b001,

    X = 0b010,
    Y = 0b011,

    Inline = 0b110,
    Block = 0b111,
}

/// Specifies which tracks(s) on the axis that the position-area span occupies.
/// Represented as 3 bits: start, center, end track.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionAreaTrack {
    /// First track
    Start = 0b001,
    /// First and center.
    SpanStart = 0b011,
    /// Last track.
    End = 0b100,
    /// Last and center.
    SpanEnd = 0b110,
    /// Center track.
    Center = 0b010,
    /// All tracks
    SpanAll = 0b111,
}

pub const AXIS_SHIFT: usize = 3;
pub const SELF_WM_SHIFT: usize = 6;
pub const SELF_WM: u8 = 1u8 << 6;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[allow(missing_docs)]
#[repr(u8)]
/// Possible values for the `position-area` property's keywords.
/// Represented by [0z xxx yyy], where z means "self wm resolution", xxx is the type (as in
/// PositionAreaAxis and yyy is the PositionAreaTrack
/// https://drafts.csswg.org/css-anchor-position-1/#propdef-position-area
pub enum PositionAreaKeyword {
    #[default]
    None = 0,

    Center = PositionAreaTrack::Center as u8,
    SpanAll = PositionAreaTrack::SpanAll as u8,

    Start = PositionAreaTrack::Start as u8,
    End = PositionAreaTrack::End as u8,
    SpanStart = PositionAreaTrack::SpanStart as u8,
    SpanEnd = PositionAreaTrack::SpanEnd as u8,

    Top = ((PositionAreaAxis::Vertical as u8) << AXIS_SHIFT) | PositionAreaTrack::Start as u8,
    Bottom = ((PositionAreaAxis::Vertical as u8) << AXIS_SHIFT) | PositionAreaTrack::End as u8,
}

#[no_mangle]
extern "C" fn root(_: PositionAreaKeyword, _: PositionAreaTrack, _: PositionAreaAxis) {}

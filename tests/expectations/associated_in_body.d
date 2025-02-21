module cbindgen;

@nogc nothrow @safe:

/// An arbitrary identifier for a native (OS compositor) surface
struct NativeSurfaceId {
  @disable this();
  ulong _0;
}
/// A special id for the native surface that is used for debug / profiler overlays.
enum NativeSurfaceId_DEBUG_OVERLAY = NativeSurfaceId(_0: UINT64_MAX);

struct NativeTileId {
  @disable this();
  NativeSurfaceId surface_id;
  int x;
  int y;
}
/// A special id for the native surface that is used for debug / profiler overlays.
enum NativeTileId_DEBUG_OVERLAY = NativeTileId(surface_id: NativeSurfaceId_DEBUG_OVERLAY, x: 0, y: 0);

extern(C) {

void root(AlignFlags flags, NativeTileId tile);

}  // extern(C)

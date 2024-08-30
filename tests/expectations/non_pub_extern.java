import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Not implemented yet : Static { path: Path { name: "FIRST" }, export_name: "FIRST", ty: Primitive(Integer { zeroable: true, signed: false, kind: B32 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  /* Not implemented yet : Static { path: Path { name: "RENAMED" }, export_name: "RENAMED", ty: Primitive(Integer { zeroable: true, signed: false, kind: B32 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  void first();

  void renamed();

}
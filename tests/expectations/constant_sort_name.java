import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant A */


  /* Unsupported literal for constant B */


  /* Not implemented yet : Static { path: Path { name: "C" }, export_name: "C", ty: Primitive(Integer { zeroable: true, signed: false, kind: B8 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  /* Not implemented yet : Static { path: Path { name: "D" }, export_name: "D", ty: Primitive(Integer { zeroable: true, signed: false, kind: B8 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

}
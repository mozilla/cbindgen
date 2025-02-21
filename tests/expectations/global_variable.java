import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Not implemented yet : Static { path: Path { name: "MUT_GLOBAL_ARRAY" }, export_name: "MUT_GLOBAL_ARRAY", ty: Array(Primitive(Char), Value("128")), mutable: true, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  /* Not implemented yet : Static { path: Path { name: "CONST_GLOBAL_ARRAY" }, export_name: "CONST_GLOBAL_ARRAY", ty: Array(Primitive(Char), Value("128")), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

}
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class Bar extends PointerType {
    public Bar() {
      super(null);
    }
    public Bar(Pointer p) {
      super(p);
    }
  }

  class BarByReference extends Bar {
    public BarByReference() {
      super(null);
    }
    public BarByReference(Pointer p) {
      super(p);
    }
  }


  class Foo extends Structure implements Structure.ByValue {
    public Foo() {
      super();
    }

    public Foo(Pointer p) {
      super(p);
    }


  }

  class FooByReference extends Structure implements Structure.ByReference {
    public FooByReference() {
      super();
    }

    public FooByReference(Pointer p) {
      super(p);
    }


  }


  /* Not implemented yet : Static { path: Path { name: "NUMBER" }, export_name: "NUMBER", ty: Primitive(Integer { zeroable: true, signed: true, kind: B32 }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  /* Not implemented yet : Static { path: Path { name: "FOO" }, export_name: "FOO", ty: Path(GenericPath { path: Path { name: "Foo" }, export_name: "Foo", generics: [], ctype: None }), mutable: true, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  /* Not implemented yet : Static { path: Path { name: "BAR" }, export_name: "BAR", ty: Path(GenericPath { path: Path { name: "Bar" }, export_name: "Bar", generics: [], ctype: None }), mutable: false, cfg: None, annotations: AnnotationSet { annotations: {}, must_use: false, deprecated: None }, documentation: Documentation { doc_comment: [] } } */

  void root();

}
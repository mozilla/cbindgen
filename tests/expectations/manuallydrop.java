
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  class NotReprC_Point extends PointerType {
    public NotReprC_Point() {
      super(null);
    }
    public NotReprC_Point(Pointer p) {
      super(p);
    }
  }

  class NotReprC_PointByReference extends NotReprC_Point {
    public NotReprC_PointByReference() {
      super(null);
    }
    public NotReprC_PointByReference(Pointer p) {
      super(p);
    }
  }

  class Foo extends NotReprC_Point {
    public Foo() {
      super();
    }
    public Foo(Pointer p) {
      super(p);
    }
  }

  class FooByReference extends NotReprC_PointByReference {
    public FooByReference() {
      super();
    }
    public FooByReference(Pointer p) {
      super(p);
    }
  }


  @Structure.FieldOrder({"x", "y"})
  class Point extends Structure implements Structure.ByValue {
    public Point() {
      super();
    }

    public Point(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }

  @Structure.FieldOrder({"x", "y"})
  class PointByReference extends Structure implements Structure.ByReference {
    public PointByReference() {
      super();
    }

    public PointByReference(Pointer p) {
      super(p);
    }

    public int x;
    public int y;

  }



  @Structure.FieldOrder({"point"})
  class MyStruct extends Structure implements Structure.ByValue {
    public MyStruct() {
      super();
    }

    public MyStruct(Pointer p) {
      super(p);
    }

    public Point point;

  }

  @Structure.FieldOrder({"point"})
  class MyStructByReference extends Structure implements Structure.ByReference {
    public MyStructByReference() {
      super();
    }

    public MyStructByReference(Pointer p) {
      super(p);
    }

    public Point point;

  }


  void root(FooByReference a, MyStructByReference with_manual_drop);

  void take(Point with_manual_drop);

}
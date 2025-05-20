import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  interface A extends Callback {
    void invoke();
  }

  interface B extends Callback {
    void invoke();
  }

  interface C extends Callback {
    _Boolean invoke(int arg0, int arg1);
  }

  interface D extends Callback {
    Callback invoke(int arg0);
  }

  interface E extends Callback {
    Pointer invoke();
  }

  class F extends PointerType {
    public F() {
      super(null);
    }
    public F(Pointer p) {
      super(p);
    }
  }

  class FByReference extends F {
    public FByReference() {
      super(null);
    }
    public FByReference(Pointer p) {
      super(p);
    }
  }

  class G extends PointerType {
    public G() {
      super(null);
    }
    public G(Pointer p) {
      super(p);
    }
  }

  class GByReference extends G {
    public GByReference() {
      super(null);
    }
    public GByReference(Pointer p) {
      super(p);
    }
  }

  class H extends PointerType {
    public H() {
      super(null);
    }
    public H(Pointer p) {
      super(p);
    }
  }

  class HByReference extends H {
    public HByReference() {
      super(null);
    }
    public HByReference(Pointer p) {
      super(p);
    }
  }

  class I extends PointerType {
    public I() {
      super(null);
    }
    public I(Pointer p) {
      super(p);
    }
  }

  class IByReference extends I {
    public IByReference() {
      super(null);
    }
    public IByReference(Pointer p) {
      super(p);
    }
  }

  class J extends PointerType {
    public J() {
      super(null);
    }
    public J(Pointer p) {
      super(p);
    }
  }

  class JByReference extends J {
    public JByReference() {
      super(null);
    }
    public JByReference(Pointer p) {
      super(p);
    }
  }

  class K extends PointerType {
    public K() {
      super(null);
    }
    public K(Pointer p) {
      super(p);
    }
  }

  class KByReference extends K {
    public KByReference() {
      super(null);
    }
    public KByReference(Pointer p) {
      super(p);
    }
  }

  class L extends PointerType {
    public L() {
      super(null);
    }
    public L(Pointer p) {
      super(p);
    }
  }

  class LByReference extends L {
    public LByReference() {
      super(null);
    }
    public LByReference(Pointer p) {
      super(p);
    }
  }

  class M extends PointerType {
    public M() {
      super(null);
    }
    public M(Pointer p) {
      super(p);
    }
  }

  class MByReference extends M {
    public MByReference() {
      super(null);
    }
    public MByReference(Pointer p) {
      super(p);
    }
  }

  class N extends PointerType {
    public N() {
      super(null);
    }
    public N(Pointer p) {
      super(p);
    }
  }

  class NByReference extends N {
    public NByReference() {
      super(null);
    }
    public NByReference(Pointer p) {
      super(p);
    }
  }

  interface P extends Callback {
    void invoke(int named1st, _Boolean arg1, _Boolean named3rd, int arg3);
  }

  Callback O();

  void root(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l, M m, N n, P p);

  class _Boolean extends IntegerType {
    public _Boolean() {
      super(1, true);
    }

    public _Boolean(long value) {
      super(1, value, true);
    }

    public _Boolean(Pointer p) {
      this(p.getByte(0));
    }

    public static final _Boolean FALSE = new _Boolean(0);
    public static final _Boolean TRUE = new _Boolean(1);
  }

  class _BooleanByReference extends ByReference {
    public _BooleanByReference() {
      super(1);
    }

    public _BooleanByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public _Boolean getValue() {
      Pointer p = getPointer();
      return new _Boolean(p.getByte(0));
    }

    public void setValue(_Boolean value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }

}
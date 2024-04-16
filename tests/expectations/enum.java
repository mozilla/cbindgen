
import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  class A extends IntegerType {
    public A() {
      super(4, true);
    }

    public A(long value) {
      super(4, value, true);
    }

    public A(Pointer p) {
      this(p.getInt(0));
    }
    public static final A a1 = new A(0);
    public static final A a2 = new A(2);
    public static final A a3 = new A(3);
    public static final A a4 = new A(5);

  }

  class AByReference extends ByReference {
    public AByReference() {
      super(4);
    }

    public AByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public A getValue() {
      Pointer p = getPointer();
      return new A(p.getInt(0));
    }

    public void setValue(A value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class B extends IntegerType {
    public B() {
      super(4, true);
    }

    public B(long value) {
      super(4, value, true);
    }

    public B(Pointer p) {
      this(p.getInt(0));
    }
    public static final B b1 = new B(0);
    public static final B b2 = new B(2);
    public static final B b3 = new B(3);
    public static final B b4 = new B(5);

  }

  class BByReference extends ByReference {
    public BByReference() {
      super(4);
    }

    public BByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public B getValue() {
      Pointer p = getPointer();
      return new B(p.getInt(0));
    }

    public void setValue(B value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class C extends IntegerType {
    public C() {
      super(4, true);
    }

    public C(long value) {
      super(4, value, true);
    }

    public C(Pointer p) {
      this(p.getInt(0));
    }
    public static final C c1 = new C(0);
    public static final C c2 = new C(2);
    public static final C c3 = new C(3);
    public static final C c4 = new C(5);

  }

  class CByReference extends ByReference {
    public CByReference() {
      super(4);
    }

    public CByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public C getValue() {
      Pointer p = getPointer();
      return new C(p.getInt(0));
    }

    public void setValue(C value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class D extends IntegerType {
    public D() {
      super(4, true);
    }

    public D(long value) {
      super(4, value, true);
    }

    public D(Pointer p) {
      this(p.getInt(0));
    }
    public static final D d1 = new D(0);
    public static final D d2 = new D(2);
    public static final D d3 = new D(3);
    public static final D d4 = new D(5);

  }

  class DByReference extends ByReference {
    public DByReference() {
      super(4);
    }

    public DByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public D getValue() {
      Pointer p = getPointer();
      return new D(p.getInt(0));
    }

    public void setValue(D value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class E extends IntegerType {
    public E() {
      super(4, true);
    }

    public E(long value) {
      super(4, value, true);
    }

    public E(Pointer p) {
      this(p.getInt(0));
    }
    public static final E e1 = new E(0);
    public static final E e2 = new E(2);
    public static final E e3 = new E(3);
    public static final E e4 = new E(5);

  }

  class EByReference extends ByReference {
    public EByReference() {
      super(4);
    }

    public EByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public E getValue() {
      Pointer p = getPointer();
      return new E(p.getInt(0));
    }

    public void setValue(E value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class F extends IntegerType {
    public F() {
      super(4, true);
    }

    public F(long value) {
      super(4, value, true);
    }

    public F(Pointer p) {
      this(p.getInt(0));
    }
    public static final F f1 = new F(0);
    public static final F f2 = new F(2);
    public static final F f3 = new F(3);
    public static final F f4 = new F(5);

  }

  class FByReference extends ByReference {
    public FByReference() {
      super(4);
    }

    public FByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public F getValue() {
      Pointer p = getPointer();
      return new F(p.getInt(0));
    }

    public void setValue(F value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class L extends IntegerType {
    public L() {
      super(4, true);
    }

    public L(long value) {
      super(4, value, true);
    }

    public L(Pointer p) {
      this(p.getInt(0));
    }
    public static final L l1 = new L(1);
    public static final L l2 = new L(2);
    public static final L l3 = new L(3);
    public static final L l4 = new L(4);

  }

  class LByReference extends ByReference {
    public LByReference() {
      super(4);
    }

    public LByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public L getValue() {
      Pointer p = getPointer();
      return new L(p.getInt(0));
    }

    public void setValue(L value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class M extends IntegerType {
    public M() {
      super(4, true);
    }

    public M(long value) {
      super(4, value, true);
    }

    public M(Pointer p) {
      this(p.getInt(0));
    }
    public static final M m1 = new M(1);
    public static final M m2 = new M(0);
    public static final M m3 = new M(1);

  }

  class MByReference extends ByReference {
    public MByReference() {
      super(4);
    }

    public MByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public M getValue() {
      Pointer p = getPointer();
      return new M(p.getInt(0));
    }

    public void setValue(M value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class N extends IntegerType {
    public N() {
      super(4, true);
    }

    public N(long value) {
      super(4, value, true);
    }

    public N(Pointer p) {
      this(p.getInt(0));
    }
    public static final N n1 = new N(1);
    public static final N n2 = new N(2);
    public static final N n3 = new N(3);
    public static final N n4 = new N(4);

  }

  class NByReference extends ByReference {
    public NByReference() {
      super(4);
    }

    public NByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public N getValue() {
      Pointer p = getPointer();
      return new N(p.getInt(0));
    }

    public void setValue(N value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class O extends IntegerType {
    public O() {
      super(4, true);
    }

    public O(long value) {
      super(4, value, true);
    }

    public O(Pointer p) {
      this(p.getInt(0));
    }
    public static final O o1 = new O(1);
    public static final O o2 = new O(2);
    public static final O o3 = new O(3);
    public static final O o4 = new O(4);

  }

  class OByReference extends ByReference {
    public OByReference() {
      super(4);
    }

    public OByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public O getValue() {
      Pointer p = getPointer();
      return new O(p.getInt(0));
    }

    public void setValue(O value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
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

  class Opaque extends PointerType {
    public Opaque() {
      super(null);
    }
    public Opaque(Pointer p) {
      super(p);
    }
  }

  class OpaqueByReference extends Opaque {
    public OpaqueByReference() {
      super(null);
    }
    public OpaqueByReference(Pointer p) {
      super(p);
    }
  }


  class G extends IntegerType {
    public G() {
      super(4, true);
    }

    public G(long value) {
      super(4, value, true);
    }

    public G(Pointer p) {
      this(p.getInt(0));
    }
    public static final G Foo = new G(1);
    public static final G Bar = new G(2);
    public static final G Baz = new G(3);

  }

  class GByReference extends ByReference {
    public GByReference() {
      super(4);
    }

    public GByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public G getValue() {
      Pointer p = getPointer();
      return new G(p.getInt(0));
    }

    public void setValue(G value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class H extends IntegerType {
    public H() {
      super(4, true);
    }

    public H(long value) {
      super(4, value, true);
    }

    public H(Pointer p) {
      this(p.getInt(0));
    }
    public static final H H_Foo = new H(1);
    public static final H H_Bar = new H(2);
    public static final H H_Baz = new H(3);

  }

  class HByReference extends ByReference {
    public HByReference() {
      super(4);
    }

    public HByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public H getValue() {
      Pointer p = getPointer();
      return new H(p.getInt(0));
    }

    public void setValue(H value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class ExI extends IntegerType {
    public ExI() {
      super(4, true);
    }

    public ExI(long value) {
      super(4, value, true);
    }

    public ExI(Pointer p) {
      this(p.getInt(0));
    }
    public static final ExI ExI_Foo = new ExI(1);
    public static final ExI ExI_Bar = new ExI(2);
    public static final ExI ExI_Baz = new ExI(3);

  }

  class ExIByReference extends ByReference {
    public ExIByReference() {
      super(4);
    }

    public ExIByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public ExI getValue() {
      Pointer p = getPointer();
      return new ExI(p.getInt(0));
    }

    public void setValue(ExI value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class P extends IntegerType {
    public P() {
      super(4, true);
    }

    public P(long value) {
      super(4, value, true);
    }

    public P(Pointer p) {
      this(p.getInt(0));
    }
    public static final P P0 = new P(1);
    public static final P P1 = new P(2);

  }

  class PByReference extends ByReference {
    public PByReference() {
      super(4);
    }

    public PByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public P getValue() {
      Pointer p = getPointer();
      return new P(p.getInt(0));
    }

    public void setValue(P value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class Q extends IntegerType {
    public Q() {
      super(4, true);
    }

    public Q(long value) {
      super(4, value, true);
    }

    public Q(Pointer p) {
      this(p.getInt(0));
    }
    public static final Q Ok = new Q(1);
    public static final Q Err = new Q(2);

  }

  class QByReference extends ByReference {
    public QByReference() {
      super(4);
    }

    public QByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public Q getValue() {
      Pointer p = getPointer();
      return new Q(p.getInt(0));
    }

    public void setValue(Q value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }



  class R extends IntegerType {
    public R() {
      super(4, true);
    }

    public R(long value) {
      super(4, value, true);
    }

    public R(Pointer p) {
      this(p.getInt(0));
    }
    public static final R IRFoo = new R(1);
    public static final R IRBar = new R(2);
    public static final R IRBaz = new R(3);

  }

  class RByReference extends ByReference {
    public RByReference() {
      super(4);
    }

    public RByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public R getValue() {
      Pointer p = getPointer();
      return new R(p.getInt(0));
    }

    public void setValue(R value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  void root(OpaqueByReference opaque, 
            A a, 
            B b, 
            C c, 
            D d, 
            E e, 
            F f, 
            G g, 
            H h, 
            ExI i, 
            J j, 
            K k, 
            L l, 
            M m, 
            N n, 
            O o, 
            P p, 
            Q q, 
            R r);

}


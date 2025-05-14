import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  interface VaListFnPtr extends Callback {
    int invoke(int count, Pointer arg1);
  }

  interface VaListFnPtr2 extends Callback {
    int invoke(int count, Pointer arg1);
  }


  @Structure.FieldOrder({"fn1"})
  class Interface_______i32_______i32_______va_list extends Structure implements Structure.ByValue {
    public Interface_______i32_______i32_______va_list() {
      super();
    }

    public Interface_______i32_______i32_______va_list(Pointer p) {
      super(p);
    }

    public Callback fn1;

  }

  @Structure.FieldOrder({"fn1"})
  class Interface_______i32_______i32_______va_listByReference extends Structure implements Structure.ByReference {
    public Interface_______i32_______i32_______va_listByReference() {
      super();
    }

    public Interface_______i32_______i32_______va_listByReference(Pointer p) {
      super(p);
    }

    public Callback fn1;

  }


  int va_list_test(int count, Pointer arg1);

  int va_list_test2(int count, Pointer arg1);

  void va_list_fn_ptrs(Callback fn1, 
                       Callback fn2, 
                       VaListFnPtr fn3, 
                       VaListFnPtr2 fn4, 
                       Interface_______i32_______i32_______va_list fn5, 
                       Interface_______i32_______i32_______va_list fn6);

}
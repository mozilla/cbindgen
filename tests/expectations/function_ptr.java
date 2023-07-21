import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  interface MyCallback extends Callback {
    void invoke(NativeLong a, NativeLong b);
  }

  interface MyOtherCallback extends Callback {
    void invoke(NativeLong a, 
                NativeLong lot, 
                NativeLong of, 
                NativeLong args, 
                NativeLong and_then_some);
  }

  void my_function(MyCallback a, MyOtherCallback b);

}
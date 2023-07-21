import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  void ptr_as_array(int n, IntByReference arg, LongByReference v);

  void ptr_as_array1(int n, IntByReference arg, LongByReference v);

  void ptr_as_array2(int n, IntByReference arg, LongByReference v);

  void ptr_as_array_wrong_syntax(IntByReference arg, IntByReference v, IntByReference arg2);

  void ptr_as_array_unnamed(IntByReference arg0, IntByReference arg1);

}
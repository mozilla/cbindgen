import com.sun.jna.*; // manually added
import com.sun.jna.ptr.*; // manually added


enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  void root();

}
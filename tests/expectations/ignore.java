import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant NO_IGNORE_CONST */


  /* Unsupported literal for constant NO_IGNORE_INNER_CONST */


  void no_ignore_root();

  void no_ignore_associated_method();

}
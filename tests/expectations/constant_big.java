import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  /* Unsupported literal for constant UNSIGNED_NEEDS_ULL_SUFFIX */


  /* Unsupported literal for constant UNSIGNED_DOESNT_NEED_ULL_SUFFIX */


  /* Unsupported literal for constant SIGNED_NEEDS_ULL_SUFFIX */


  /* Unsupported literal for constant SIGNED_DOESNT_NEED_ULL_SUFFIX */


}
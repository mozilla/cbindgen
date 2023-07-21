import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  /**
   * The root of all evil.
   *
   * But at least it contains some more documentation as someone would expect
   * from a simple test case like this. Though, this shouldn't appear in the
   * output.
   */
  void root();


  /**
   * A little above the root, and a lot more visible, with a run-on sentence
   * to test going over the first line.
   *
   * Still not here, though.
   */
  void trunk();

}
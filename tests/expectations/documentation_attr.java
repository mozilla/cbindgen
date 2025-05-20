import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  /**
   *With doc attr, each attr contribute to one line of document
   *like this one with a new line character at its end
   *and this one as well. So they are in the same paragraph
   *
   *Line ends with one new line should not break
   *
   *Line ends with two spaces and a new line
   *should break to next line
   *
   *Line ends with two new lines
   *
   *Should break to next paragraph
   */
  void root();

}
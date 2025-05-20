import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  void test_camel_case(int fooBar);

  void test_pascal_case(int FooBar);

  void test_snake_case(int foo_bar);

  void test_screaming_snake_case(int FOO_BAR);

  void test_gecko_case(int aFooBar);

}
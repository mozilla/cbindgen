import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;


  @Structure.FieldOrder({"x", "y", "w", "h"})
  class Rect extends Structure implements Structure.ByValue {
    public Rect() {
      super();
    }

    public Rect(Pointer p) {
      super(p);
    }

    public float x;
    public float y;
    public float w;
    public float h;

  }

  @Structure.FieldOrder({"x", "y", "w", "h"})
  class RectByReference extends Structure implements Structure.ByReference {
    public RectByReference() {
      super();
    }

    public RectByReference(Pointer p) {
      super(p);
    }

    public float x;
    public float y;
    public float w;
    public float h;

  }



  @Structure.FieldOrder({"r", "g", "b", "a"})
  class Color extends Structure implements Structure.ByValue {
    public Color() {
      super();
    }

    public Color(Pointer p) {
      super(p);
    }

    public byte r;
    public byte g;
    public byte b;
    public byte a;

  }

  @Structure.FieldOrder({"r", "g", "b", "a"})
  class ColorByReference extends Structure implements Structure.ByReference {
    public ColorByReference() {
      super();
    }

    public ColorByReference(Pointer p) {
      super(p);
    }

    public byte r;
    public byte g;
    public byte b;
    public byte a;

  }



  class DisplayItem extends IntegerType {
    public DisplayItem() {
      super(4, true);
    }

    public DisplayItem(long value) {
      super(4, value, true);
    }

    public DisplayItem(Pointer p) {
      this(p.getInt(0));
    }
    public static final DisplayItem Fill = new DisplayItem(1);
    public static final DisplayItem Image = new DisplayItem(2);
    public static final DisplayItem ClearScreen = new DisplayItem(3);

  }

  class DisplayItemByReference extends ByReference {
    public DisplayItemByReference() {
      super(4);
    }

    public DisplayItemByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public DisplayItem getValue() {
      Pointer p = getPointer();
      return new DisplayItem(p.getInt(0));
    }

    public void setValue(DisplayItem value) {
      Pointer p = getPointer();
      p.setInt(0, value.intValue());
    }

  }


  _Boolean push_item(DisplayItem item);

  class _Boolean extends IntegerType {
    public _Boolean() {
      super(1, true);
    }

    public _Boolean(long value) {
      super(1, value, true);
    }

    public _Boolean(Pointer p) {
      this(p.getByte(0));
    }

    public static final _Boolean FALSE = new _Boolean(0);
    public static final _Boolean TRUE = new _Boolean(1);
  }

  class _BooleanByReference extends ByReference {
    public _BooleanByReference() {
      super(1);
    }

    public _BooleanByReference(Pointer p) {
      super(1);
      setPointer(p);
    }

    public _Boolean getValue() {
      Pointer p = getPointer();
      return new _Boolean(p.getByte(0));
    }

    public void setValue(_Boolean value) {
      Pointer p = getPointer();
      p.setByte(0, (byte)value.intValue());
    }

  }

}
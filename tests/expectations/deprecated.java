import com.sun.jna.*;
import com.sun.jna.ptr.*;

enum BindingsSingleton {
  INSTANCE;
  final Bindings lib = Native.load("", Bindings.class);
}

interface Bindings extends Library {
  Bindings INSTANCE = BindingsSingleton.INSTANCE.lib;

  @Deprecated
  class DeprecatedEnum extends IntegerType {
    public DeprecatedEnum() {
      super(4);
    }

    public DeprecatedEnum(long value) {
      super(4, value);
    }

    public DeprecatedEnum(Pointer p) {
      this(p.getInt(0));
    }
    public static final DeprecatedEnum A = new DeprecatedEnum(0);

  }

  class DeprecatedEnumByReference extends ByReference {
    public DeprecatedEnumByReference() {
      super(4);
    }

    public DeprecatedEnumByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public DeprecatedEnum getValue() {
      return new DeprecatedEnum(getPointer().getInt(0));
    }

    public void setValue(DeprecatedEnum value) {
      getPointer().setInt(0, value.intValue());
    }

  }

  /**
   * @deprecated This is a note
   */
  @Deprecated
  class DeprecatedEnumWithNote extends IntegerType {
    public DeprecatedEnumWithNote() {
      super(4);
    }

    public DeprecatedEnumWithNote(long value) {
      super(4, value);
    }

    public DeprecatedEnumWithNote(Pointer p) {
      this(p.getInt(0));
    }
    public static final DeprecatedEnumWithNote B = new DeprecatedEnumWithNote(0);

  }

  class DeprecatedEnumWithNoteByReference extends ByReference {
    public DeprecatedEnumWithNoteByReference() {
      super(4);
    }

    public DeprecatedEnumWithNoteByReference(Pointer p) {
      super(4);
      setPointer(p);
    }

    public DeprecatedEnumWithNote getValue() {
      return new DeprecatedEnumWithNote(getPointer().getInt(0));
    }

    public void setValue(DeprecatedEnumWithNote value) {
      getPointer().setInt(0, value.intValue());
    }

  }


  @Deprecated
  @Structure.FieldOrder({"a"})
  class DeprecatedStruct extends Structure implements Structure.ByValue {
    public DeprecatedStruct() {
      super();
    }

    public DeprecatedStruct(Pointer p) {
      super(p);
    }

    public int a;

  }

  @Deprecated
  @Structure.FieldOrder({"a"})
  class DeprecatedStructByReference extends Structure implements Structure.ByReference {
    public DeprecatedStructByReference() {
      super();
    }

    public DeprecatedStructByReference(Pointer p) {
      super(p);
    }

    public int a;

  }



  /**
   * @deprecated This is a note
   */
  @Deprecated
  @Structure.FieldOrder({"a"})
  class DeprecatedStructWithNote extends Structure implements Structure.ByValue {
    public DeprecatedStructWithNote() {
      super();
    }

    public DeprecatedStructWithNote(Pointer p) {
      super(p);
    }

    public int a;

  }

  /**
   * @deprecated This is a note
   */
  @Deprecated
  @Structure.FieldOrder({"a"})
  class DeprecatedStructWithNoteByReference extends Structure implements Structure.ByReference {
    public DeprecatedStructWithNoteByReference() {
      super();
    }

    public DeprecatedStructWithNoteByReference(Pointer p) {
      super(p);
    }

    public int a;

  }


  @Deprecated
  void deprecated_without_note();

  /**
   * @deprecated This is a note
   */
  @Deprecated
  void deprecated_without_bracket();

  /**
   * @deprecated This is a note
   */
  @Deprecated
  void deprecated_with_note();

  /**
   * @deprecated This is a note
   */
  @Deprecated
  void deprecated_with_note_and_since();

  /**
   * @deprecated This quote " requires to be quoted, and this [
] requires to be escaped
   */
  @Deprecated
  void deprecated_with_note_which_requires_to_be_escaped();

  void dummy(DeprecatedEnum a, 
             DeprecatedEnumWithNote b, 
             DeprecatedStruct c, 
             DeprecatedStructWithNote d);

}
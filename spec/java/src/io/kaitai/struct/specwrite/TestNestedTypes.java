package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.NestedTypes;
import org.testng.annotations.Test;

public class TestNestedTypes extends CommonSpec {
    @Test
    public void testNestedTypes() throws Exception {
        NestedTypes r = new NestedTypes();

        NestedTypes.SubtypeA one = new NestedTypes.SubtypeA();

        NestedTypes.SubtypeB one1 = new NestedTypes.SubtypeB();
        one1.setValueB((byte) 80);

        NestedTypes.SubtypeA.SubtypeC one2 = new NestedTypes.SubtypeA.SubtypeC();
        one2.setValueC((byte) 65);

        one.setTypedAtRoot(one1);
        one.setTypedHere(one2);

        NestedTypes.SubtypeB two = new NestedTypes.SubtypeB();
        two.setValueB((byte) 67);

        r.setOne(one);
        r.setTwo(two);

        assertEqualToFile(r, "fixed_struct.bin");
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.IfStruct;
import org.testng.annotations.Test;

public class TestIfStruct extends CommonSpec {
    @Test
    public void testIfStruct() throws Exception {
        IfStruct r = new IfStruct();

        IfStruct.Operation op1 = new IfStruct.Operation() {{
            setOpcode(0x53);
            setArgStr(new IfStruct.ArgStr() {{
                setStr("foo");

                // to be set automatically
                setLen(3);
            }});
        }};
        r.setOp1(op1);

        IfStruct.Operation op2 = new IfStruct.Operation() {{
            setOpcode(0x54);
            setArgTuple(new IfStruct.ArgTuple() {{
                setNum1(0x42);
                setNum2(0x43);
            }});
        }};
        r.setOp2(op2);

        IfStruct.Operation op3 = new IfStruct.Operation() {{
            setOpcode(0x53);
            setArgStr(new IfStruct.ArgStr() {{
                setStr("bar");

                // to be set automatically
                setLen(3);
            }});
        }};
        r.setOp3(op3);

        assertEqualToFullFile(r, "if_struct.bin");
    }
}

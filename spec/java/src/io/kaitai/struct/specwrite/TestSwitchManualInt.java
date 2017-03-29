package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.SwitchManualInt;
import org.testng.annotations.Test;

import java.util.ArrayList;

public class TestSwitchManualInt extends CommonSpec {
    @Test
    public void testSwitchManualInt() throws Exception {
        SwitchManualInt r = new SwitchManualInt();

        ArrayList<SwitchManualInt.Opcode> opcodes = new ArrayList<>();

        opcodes.add(new SwitchManualInt.Opcode() {{
            setCode(83);
            setBody(new Strval() {{
                setValue("foobar");
            }});
        }});

        opcodes.add(new SwitchManualInt.Opcode() {{
            setCode(73);
            setBody(new Intval() {{
                setValue(0x42);
            }});
        }});

        opcodes.add(new SwitchManualInt.Opcode() {{
            setCode(73);
            setBody(new Intval() {{
                setValue(0x37);
            }});
        }});

        opcodes.add(new SwitchManualInt.Opcode() {{
            setCode(83);
            setBody(new Strval() {{
                setValue("");
            }});
        }});

        r.setOpcodes(opcodes);

        assertEqualToFullFile(r, "switch_opcodes.bin");
    }
}

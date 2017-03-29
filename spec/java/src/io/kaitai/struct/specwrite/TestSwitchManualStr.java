package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.SwitchManualStr;
import io.kaitai.struct.testwrite.SwitchManualStr.Opcode.Intval;
import io.kaitai.struct.testwrite.SwitchManualStr.Opcode.Strval;
import org.testng.annotations.Test;

import java.util.ArrayList;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualStr extends CommonSpec {
    @Test
    public void testSwitchManualStr() throws Exception {
        SwitchManualStr r = new SwitchManualStr();

        ArrayList<SwitchManualStr.Opcode> opcodes = new ArrayList<>();

        opcodes.add(new SwitchManualStr.Opcode() {{
            setCode("S");
            setBody(new Strval() {{
                setValue("foobar");
            }});
        }});

        opcodes.add(new SwitchManualStr.Opcode() {{
            setCode("I");
            setBody(new Intval() {{
                setValue(0x42);
            }});
        }});

        opcodes.add(new SwitchManualStr.Opcode() {{
            setCode("I");
            setBody(new Intval() {{
                setValue(0x37);
            }});
        }});

        opcodes.add(new SwitchManualStr.Opcode() {{
            setCode("S");
            setBody(new Strval() {{
                setValue("");
            }});
        }});

        r.setOpcodes(opcodes);

        assertEqualToFullFile(r, "switch_opcodes.bin");
    }
}

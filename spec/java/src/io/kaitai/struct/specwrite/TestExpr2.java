package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.Expr2;
import org.testng.annotations.Test;
import java.nio.charset.StandardCharsets;
import static org.testng.Assert.*;

public class TestExpr2 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return Expr2.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_encodings.bin";
    }

    @Test
    public void testEdit() throws Exception {
        Expr2 r = Expr2.fromFile(SRC_DIR + getSrcFilename());
        r._read();

        int oldLenMod = r.str2().lenMod();
        int oldStr2RestAvg = r.str2().rest().avg();

        r.str2().setStr("Kaitai Struct カ");
        r.str2()._invalidateLenMod();

        int str2Size = r.str2().str().getBytes(StandardCharsets.UTF_8).length;
        r.str2().setLenOrig(str2Size + 3);
        assertNotEquals(r.str2().lenMod(), oldLenMod);
        assertIntEquals(r.str2().lenMod(), str2Size);

        r.str2().rest().setByte0(0xfa);
        r.str2().rest()._invalidateAvg();
        r.str2().rest().setByte1(0x44);
        r.str2().rest().setByte2(0x88);
        assertNotEquals(r.str2().rest().avg(), oldStr2RestAvg);

        // see <CommonSpec>.testReadWriteRoundtrip
        Object origDump = dumpStruct(r);

        ByteBufferKaitaiStream newIo = new ByteBufferKaitaiStream(
            2 + // str1.len_orig
            r.str1().lenMod() + // str1.str
            3 + // str1.rest
            2 + // str2.len_orig
            r.str2().lenMod() + // str2.str
            3 // str2.rest
        );
        r._write(newIo);
        newIo.seek(0);

        Expr2 newR = new Expr2(newIo);
        newR._read();

        Object newDump = dumpStruct(newR);

        assertEquals(newDump, origDump);
    }
}

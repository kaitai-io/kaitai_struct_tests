package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct;

import java.io.FileInputStream;
import java.io.IOException;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertTrue;

public class CommonSpec {
    private final static String SRC_DIR = io.kaitai.struct.spec.CommonSpec.SRC_DIR;

    protected void assertEqualToFullFile(KaitaiStruct.ReadWrite struct, String fn) throws IOException {
        byte[] actual = structToByteArray(struct);

        KaitaiStream expFile = new KaitaiStream(SRC_DIR + fn);
        byte[] expected = expFile.readBytesFull();
        expFile.close();

        assertEquals(byteArrayToHex(actual), byteArrayToHex(expected));
    }

    protected void assertEqualToFile(KaitaiStruct.ReadWrite struct, String fn) throws IOException {
        byte[] actual = structToByteArray(struct);

        assertTrue(actual.length > 0, "no data was written");

        FileInputStream fis = new FileInputStream(SRC_DIR + fn);
        byte[] expected = new byte[actual.length];
        fis.read(expected);
        fis.close();

        assertEquals(byteArrayToHex(actual), byteArrayToHex(expected));
    }

    protected byte[] structToByteArray(KaitaiStruct.ReadWrite struct) {
        struct._check();
        KaitaiStream io = new KaitaiStream(1024 * 1024);
        struct._write(io);
        long size = io.pos();
        io.seek(0);
        return io.readBytes(size);
    }

    protected KaitaiStream structToReadStream(KaitaiStruct.ReadWrite struct) {
        return new KaitaiStream(structToByteArray(struct));
    }

    private String byteArrayToHex(byte[] arr) {
        StringBuilder sb = new StringBuilder();
        sb.append('\n');
        for (int i = 0; i < arr.length; i++) {
            if ((i % 16) != 0) {
                sb.append((i % 4 == 0) ? '|' : ' ');
            }
            sb.append(String.format("%02x", arr[i]));
            if ((i % 16) == 15)
                sb.append('\n');
        }
        sb.append('\n');
        return sb.toString();
    }
}

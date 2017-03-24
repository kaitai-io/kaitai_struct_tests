package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct;

import java.io.FileInputStream;
import java.io.IOException;

import static org.testng.Assert.assertEquals;

public class CommonSpec {
    private final static String SRC_DIR = io.kaitai.struct.spec.CommonSpec.SRC_DIR;

    protected KaitaiStream emptyIO() {
        return new KaitaiStream(new byte[1024 * 1024]);
    }

    protected void assertEqualToFile(KaitaiStruct struct, String fn) throws IOException {
        KaitaiStream io = struct._io();
        int size = io.pos();

        io.seek(0);
        byte[] actual = io.readBytes(size);

        FileInputStream fis = new FileInputStream(SRC_DIR + fn);
        byte[] expected = new byte[size];
        fis.read(expected);

        assertEquals(byteArrayToHex(actual), byteArrayToHex(expected));
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

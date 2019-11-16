package io.kaitai.struct.testformats;

import io.kaitai.struct.CustomDecoder;

public class MyCustomFx implements CustomDecoder {
    private int key;

    public MyCustomFx(int key, boolean flag, byte[] someBytes) {
        this.key = flag ? key : -key;
    }

    @Override
    public byte[] decode(byte[] src) {
        byte[] dst = new byte[src.length];
        for (int i = 0; i < src.length; i++)
            dst[i] = (byte) (src[i] + key);
        return dst;
    }
}

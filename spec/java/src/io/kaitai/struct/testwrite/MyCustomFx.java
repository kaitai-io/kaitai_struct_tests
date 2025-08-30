package io.kaitai.struct.testwrite;

import io.kaitai.struct.CustomProcessor;

public class MyCustomFx implements CustomProcessor {
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

    @Override
    public byte[] encode(byte[] src) {
        int oldKey = key;
        key = -key;
        byte[] res = decode(src);
        key = oldKey;
        return res;
    }
}

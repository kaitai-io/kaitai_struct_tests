package io.kaitai.struct.testformats;

import io.kaitai.struct.CustomDecoder;

public class CustomFxNoArgs implements CustomDecoder {
    public CustomFxNoArgs() {
    }

    @Override
    public byte[] decode(byte[] src) {
        byte[] dst = new byte[src.length + 2];
        dst[0] = '_';
        dst[src.length + 1] = '_';
        System.arraycopy(src, 0, dst, 1, src.length);
        return dst;
    }
}

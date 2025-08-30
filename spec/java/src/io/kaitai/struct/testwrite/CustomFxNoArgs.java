package io.kaitai.struct.testwrite;

import io.kaitai.struct.CustomProcessor;

public class CustomFxNoArgs implements CustomProcessor {
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

    @Override
    public byte[] encode(byte[] src) {
        if (src.length >= 2 && src[0] == '_' && src[src.length - 1] == '_') {
            byte[] dst = new byte[src.length - 2];
            System.arraycopy(src, 1, dst, 0, dst.length);
            return dst;
        } else {
            // usually it's decode(byte[]) that puts restrictions on the input data, but
            // hey - this class is just for testing purposes anyway :-P
            throw new UnsupportedOperationException("CustomFxNoArgs can only encode data like '_(.*)_'");
        }
    }
}

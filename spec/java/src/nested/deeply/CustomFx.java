package nested.deeply;

import io.kaitai.struct.CustomDecoder;

public class CustomFx implements CustomDecoder {
    public CustomFx(int x) {
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

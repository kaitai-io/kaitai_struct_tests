package io.kaitai.struct.specwrite;

import java.io.IOException;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.IdentityHashMap;
import java.util.List;
import java.util.Map;

import static org.testng.Assert.assertEquals;
import org.testng.annotations.Test;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct;

public abstract class CommonSpec extends io.kaitai.struct.spec.CommonSpec {
    protected abstract Class<? extends KaitaiStruct.ReadWrite> getStructClass();

    protected abstract String getSrcFilename();

    @Test
    protected void testReadWriteRoundtrip() throws Exception {
        Class<? extends KaitaiStruct.ReadWrite> structClass = getStructClass();
        String fn = getSrcFilename();

        Method fromFileMethod = structClass.getMethod("fromFile", String.class);
        KaitaiStruct.ReadWrite origKs = (KaitaiStruct.ReadWrite) fromFileMethod.invoke(null, SRC_DIR + fn);
        origKs._read();

        Object origDump = dumpStruct(origKs);

        ByteBufferKaitaiStream newIo = new ByteBufferKaitaiStream(origKs._io().size());
        origKs._write(newIo);
        newIo.seek(0);

        KaitaiStruct.ReadWrite newKs = structClass
            .getConstructor(KaitaiStream.class)
            .newInstance(newIo);
        newKs._read();

        Object newDump = dumpStruct(newKs);

        // // add debug dumps for a specific test format as follows:
        // if (structClass.equals(HelloWorld.class)) {
        //     System.out.println(structClass.getSimpleName());
        //     System.out.println("origDump: " + origDump);
        //     System.out.println(" newDump: " + newDump);
        //     System.out.println();
        // }

        assertEquals(newDump, origDump);
    }

    protected void assertEqualToFullFile(KaitaiStruct.ReadWrite struct, String fn) throws IOException {
        byte[] expected = Files.readAllBytes(Paths.get(SRC_DIR + fn));
        struct._check();
        byte[] actual = new byte[expected.length];
        try (KaitaiStream actIo = new ByteBufferKaitaiStream(actual)) {
            struct._write(actIo);
        }

        assertByteArrayEquals(actual, expected);
    }

    protected void assertByteArrayEquals(byte[] actual, byte[] expected) {
        assertEquals(byteArrayToHex(actual), byteArrayToHex(expected));
    }

    protected static Object dumpStruct(KaitaiStruct.ReadWrite struct) throws Exception {
        return dumpStructValue(struct, new IdentityHashMap<>(), 50, "/");
    }

    protected static Object dumpStructValue(
            Object value,
            IdentityHashMap<KaitaiStruct.ReadWrite, String> parentStructs,
            int recursionDepthLimit,
            String currentPath
    ) throws Exception {
        if (recursionDepthLimit < 0) {
            // The reason this limit exists is that it is theoretically possible to
            // recursively parse an infinite number of new types using instances (even
            // keeping track of already encountered identical objects doesn't solve this,
            // since new objects are created each time) - not that it's a good idea to do
            // this in any KSY specification, of course, since it would make it impossible
            // to dump it to JSON, for example, but just to be safe there is this limit.
            throw new IllegalStateException(
                "recursion depth limit reached, cannot recurse into " + currentPath
            );
        }

        if (value instanceof KaitaiStruct.ReadWrite) {
            KaitaiStruct.ReadWrite struct = (KaitaiStruct.ReadWrite) value;

            Map<String, Object> dump = new HashMap<>();
            {
                String circularRefToParentPath = parentStructs.get(struct);
                if (circularRefToParentPath != null) {
                    dump.put("$ref", circularRefToParentPath);
                    return dump;
                }
            }
            parentStructs.put(struct, currentPath);
            for (final Method m : struct.getClass().getDeclaredMethods()) {
                if (!Modifier.isPublic(m.getModifiers())) continue;
                if (Modifier.isStatic(m.getModifiers())) continue; // ignore static methods, i.e. "fromFile"

                final String methodName = m.getName();
                if (methodName.startsWith("_")) {
                    if (
                        methodName.startsWith("_read") ||
                        methodName.startsWith("_check") ||
                        methodName.startsWith("_write") ||
                        methodName.startsWith("_invalidate") ||
                        methodName.startsWith("_fetchInstances") ||
                        methodName.startsWith("_raw_")
                    ) {
                        continue;
                    }
                }
                if (m.getParameterCount() != 0) continue; // we only want getters (which must have 0 args)

                // here we could maybe set `_raw_*` properties to `null` to avoid fooling the test
                // as easily as merely writing the `_raw_*` bytes left over from parsing, but so far
                // so good (it's our generated code, so we can make sure it's not trying to cheat :-P)

                Object o = m.invoke(struct);
                dump.put(methodName, dumpStructValue(
                    o, parentStructs, recursionDepthLimit - 1,
                    currentPath + (currentPath.equals("/") ? "" : "/") + methodName
                ));
            }
            // We call the `_check()` method after dumping the object. This is necessary: if we did
            // this before the object is fully dumped, some parse instances might not have been
            // invoked yet (meaning that their storage fields would be `null`, so it would fail).
            struct._check();
            parentStructs.remove(struct);
            value = dump;
        } else if (value instanceof List<?>) {
            List<?> list = (List<?>) value;
            List<Object> out = new ArrayList<>();
            for (int i = 0; i < list.size(); i++) {
                out.add(dumpStructValue(
                    list.get(i), parentStructs, recursionDepthLimit - 1,
                    currentPath + (currentPath.equals("/") ? "" : "/") + i
                ));
            }
            value = out;
        } else {
            if (value instanceof KaitaiStream) {
                value = ((KaitaiStream) value).toByteArray();
            }
            if (value instanceof byte[]) {
                StringBuilder sb = new StringBuilder();
                for (byte b : (byte[]) value) {
                    sb.append(String.format("%02x ", b));
                }
                value = "[" + sb.substring(0, sb.length() - (sb.length() != 0 && sb.charAt(sb.length() - 1) == ' ' ? 1 : 0)) + "]";
            }
        }

        return value;
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

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class SpecTests {
    private static final String SRC_DIR = "../../src/";

    @Test
    public void testHelloWorld() throws Exception {
        HelloWorld r = HelloWorld.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
    }
    
    @Test
    public void testFixedStruct() throws Exception {
        FixedStruct r = FixedStruct.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.hdr().uint8(), 255);
        assertEquals(r.hdr().uint16(), 65535);
        assertEquals(r.hdr().uint32(), 4294967295L);
        //assertEquals(r.hdr().uint64(), 18446744073709551615);
        assertEquals(r.hdr().uint64(), 0xFFFFFFFFFFFFFFFFL);

        assertEquals(r.hdr().sint8(), -1);
        assertEquals(r.hdr().sint16(), -1);
        assertEquals(r.hdr().sint32(), -1);
        assertEquals(r.hdr().sint64(), -1);

        assertEquals(r.hdr().uint16le(), 66);
        assertEquals(r.hdr().uint32le(), 66);
        assertEquals(r.hdr().uint64le(), 66);

        assertEquals(r.hdr().sint16le(), -66);
        assertEquals(r.hdr().sint32le(), -66);
        assertEquals(r.hdr().sint64le(), -66);

        assertEquals(r.hdr().uint16be(), 66);
        assertEquals(r.hdr().uint32be(), 66);
        assertEquals(r.hdr().uint64be(), 66);

        assertEquals(r.hdr().sint16be(), -66);
        assertEquals(r.hdr().sint32be(), -66);
        assertEquals(r.hdr().sint64be(), -66);
    }

    @Test
    public void testRepeatEosStruct() throws Exception {
        RepeatEosStruct r = RepeatEosStruct.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertEquals(r.chunks().size(), 2);
        assertEquals(r.chunks().get(0).offset(), 0);
        assertEquals(r.chunks().get(0).len(), 0x42);
        assertEquals(r.chunks().get(1).offset(), 0x42);
        assertEquals(r.chunks().get(1).len(), 0x815);
    }

    @Test
    public void testRepeatEosU4() throws Exception {
        RepeatEosU4 r = RepeatEosU4.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertEquals(r.numbers().toArray(), new long[] { 0, 0x42, 0x42, 0x815 });
    }

    @Test
    public void testStrEncodings() throws Exception {
        StrEncodings r = StrEncodings.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.str1(), "Some ASCII");
        assertEquals(r.str2(), "こんにちは");
        assertEquals(r.str3(), "こんにちは");
        assertEquals(r.str4(), "░▒▓");
    }
    
    @Test
    public void testTermStrz() throws Exception {
        TermStrz r = TermStrz.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.s1(), "foo");
        assertEquals(r.s2(), "bar");
        assertEquals(r.s3(), "|baz@");
    }

    @Test
    public void testRepeatNStrz() throws Exception {
        RepeatNStrz r = RepeatNStrz.fromFile(SRC_DIR + "repeat_n_strz.bin");

        assertEquals(r.qty(), 2);
        assertEquals(r.lines().toArray(), new String[] { "foo", "bar" });
    }

    @Test
    public void testRepeatNStruct() throws Exception {
        RepeatNStruct r = RepeatNStruct.fromFile(SRC_DIR + "repeat_n_struct.bin");

        assertEquals(r.qty(), 2);
        assertEquals(r.chunks().get(0).offset(), 0x10);
        assertEquals(r.chunks().get(0).len(), 0x2078);
        assertEquals(r.chunks().get(1).offset(), 0x2088);
        assertEquals(r.chunks().get(1).len(), 0xf);
    }

    @Test
    public void testIfStruct() throws Exception {
        IfStruct r = IfStruct.fromFile(SRC_DIR + "if_struct.bin");

        assertEquals(r.op1().opcode(), 0x53);
        assertEquals(r.op1().argStr().str(), "foo");

        assertEquals(r.op2().opcode(), 0x54);
        assertEquals(r.op2().argTuple().num1(), 0x42);
        assertEquals(r.op2().argTuple().num2(), 0x43);

        assertEquals(r.op3().opcode(), 0x53);
        assertEquals(r.op3().argStr().str(), "bar");
    }

    @Test
    public void testPositionAbs() throws Exception {
        PositionAbs r = PositionAbs.fromFile(SRC_DIR + "position_abs.bin");

        assertEquals(r.indexOffset(), 0x20);
        assertEquals(r.index().entry(), "foo");
    }

    @Test
    public void testBufferedStruct() throws Exception {
        BufferedStruct r = BufferedStruct.fromFile(SRC_DIR + "buffered_struct.bin");

        assertEquals(r.len1(), 0x10);

        assertEquals(r._raw_block1(), new byte[] {
                0x42, 0, 0, 0,
                0x43, 0, 0, 0,
                -1, -1, -1, -1,
                -1, -1, -1, -1,
        });
        assertEquals(r.block1().number1(), 0x42);
        assertEquals(r.block1().number2(), 0x43);

        assertEquals(r.len2(), 0x8);

        assertEquals(r._raw_block2(), new byte[] {
                0x44, 0, 0, 0,
                0x45, 0, 0, 0,
        });
        assertEquals(r.block2().number1(), 0x44);
        assertEquals(r.block2().number2(), 0x45);

        assertEquals(r.finisher(), 0xee);
    }

    @Test
    public void testProcessXorConst() throws Exception {
        ProcessXorConst r = ProcessXorConst.fromFile(SRC_DIR + "process_xor_1.bin");

        assertEquals(r.key(), 0xff);
        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }

    @Test
    public void testProcessXorValue() throws Exception {
        ProcessXorValue r = ProcessXorValue.fromFile(SRC_DIR + "process_xor_1.bin");

        assertEquals(r.key(), 0xff);
        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }

    @Test
    public void testExpr0() throws Exception {
        Expr0 r = Expr0.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.mustBeF7().intValue(), 0xf7);
        assertEquals(r.mustBeAbc123(), "abc123");
    }

    @Test
    public void testExpr1() throws Exception {
        Expr1 r = Expr1.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.lenOf1(), 10);
        assertEquals(r.lenOf1Mod().intValue(), 8);
        assertEquals(r.str1(), "Some ASC");
        assertEquals(r.str1Len().intValue(), 8);
    }

    @Test
    public void testInstanceStd() throws Exception {
        InstanceStd r = InstanceStd.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.header(), "Some ");
    }

    @Test
    public void testExpr2() throws Exception {
        Expr2 r = Expr2.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.str1().lenOrig(), 10);
        assertEquals(r.str1().lenMod().intValue(), 7);
        assertEquals(r.str1().str(), "Some AS");

        assertEquals(r.str1Len().intValue(), 7);
        assertEquals(r.str1LenMod().intValue(), 7);
        assertEquals(r.str1Byte1().intValue(), 0x49);
        assertEquals(r.str1Avg().intValue(), 0x49);
        assertEquals(r.str1Char5(), "e");

        assertEquals(r.str1Tuple5().byte0(), 0x65);
        assertEquals(r.str1Tuple5().byte1(), 0x20);
        assertEquals(r.str1Tuple5().byte2(), 0x41);
        assertEquals(r.str1Tuple5().avg().intValue(), 0x30);

        assertEquals(r.str2Tuple5().byte0(), 0x65);
        assertEquals(r.str2Tuple5().byte1(), 0x20);
        assertEquals(r.str2Tuple5().byte2(), 0x41);
        assertEquals(r.str2Tuple5().avg().intValue(), 0x30);
    }

    @Test
    public void testInstanceStdArray() throws Exception {
        InstanceStdArray r = InstanceStdArray.fromFile(SRC_DIR + "instance_std_array.bin");

        assertEquals(r.ofs(), 0x10);
        assertEquals(r.qtyEntries(), 3);
        assertEquals(r.entrySize(), 4);

        assertEquals(r.entries().size(), 3);
        assertEquals(r.entries().get(0), new byte[] { 0x11, 0x11, 0x11, 0x11 });
        assertEquals(r.entries().get(1), new byte[] { 0x22, 0x22, 0x22, 0x22 });
        assertEquals(r.entries().get(2), new byte[] { 0x33, 0x33, 0x33, 0x33 });
    }

    @Test
    public void testNavRoot() throws Exception {
        NavRoot r = NavRoot.fromFile(SRC_DIR + "nav.bin");

        assertEquals(r.header().qtyEntries(), 2);
        assertEquals(r.header().filenameLen(), 8);

        assertEquals(r.index().entries().size(), 2);
        assertEquals(r.index().entries().get(0).filename(), "FIRST___");
        assertEquals(r.index().entries().get(1).filename(), "SECOND__");
    }

    @Test
    public void testNavParent() throws Exception {
        NavParent r = NavParent.fromFile(SRC_DIR + "nav.bin");

        assertEquals(r.header().qtyEntries(), 2);
        assertEquals(r.header().filenameLen(), 8);

        assertEquals(r.index().entries().size(), 2);
        assertEquals(r.index().entries().get(0).filename(), "FIRST___");
        assertEquals(r.index().entries().get(1).filename(), "SECOND__");
    }

    @Test
    public void testInstanceUserArray() throws Exception {
        InstanceUserArray r = InstanceUserArray.fromFile(SRC_DIR + "instance_std_array.bin");

        assertEquals(r.ofs(), 0x10);
        assertEquals(r.qtyEntries(), 3);
        assertEquals(r.entrySize(), 4);

        assertEquals(r.userEntries().size(), 3);
        assertEquals(r.userEntries().get(0).word1(), 0x1111);
        assertEquals(r.userEntries().get(0).word2(), 0x1111);
        assertEquals(r.userEntries().get(1).word1(), 0x2222);
        assertEquals(r.userEntries().get(1).word2(), 0x2222);
        assertEquals(r.userEntries().get(2).word1(), 0x3333);
        assertEquals(r.userEntries().get(2).word2(), 0x3333);
    }

    @Test
    public void testEnum0() throws Exception {
        Enum0 r = Enum0.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.pet1(), Enum0.Animal.CAT);
        assertEquals(r.pet2(), Enum0.Animal.CHICKEN);
    }

    @Test
    public void testEnumIf() throws Exception {
        EnumIf r = EnumIf.fromFile(SRC_DIR + "if_struct.bin");

        assertEquals(r.op1().opcode(), EnumIf.Opcodes.A_STRING);
        assertEquals(r.op1().argStr().str(), "foo");

        assertEquals(r.op2().opcode(), EnumIf.Opcodes.A_TUPLE);
        assertEquals(r.op2().argTuple().num1(), 0x42);
        assertEquals(r.op2().argTuple().num2(), 0x43);

        assertEquals(r.op3().opcode(), EnumIf.Opcodes.A_STRING);
        assertEquals(r.op3().argStr().str(), "bar");
    }

    @Test
    public void testInstanceIoUser() throws Exception {
        InstanceIoUser r = InstanceIoUser.fromFile(SRC_DIR + "instance_io.bin");

        assertEquals(r.qtyEntries(), 3);

        assertEquals(r.entries().get(0).name(), "the");
        assertEquals(r.entries().get(1).name(), "rainy");
        assertEquals(r.entries().get(2).name(), "day it is");
    }

    @Test
    public void testDefaultBigEndian() throws Exception {
        DefaultBigEndian r = DefaultBigEndian.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.one(), 0x7000000);
    }

    @Test
    public void testZlibWithHeader78() throws Exception {
        ZlibWithHeader78 r = ZlibWithHeader78.fromFile(SRC_DIR + "zlib_with_header_78.bin");

        assertEquals(new String(r.data(), Charset.forName("UTF-8")), "a quick brown fox jumps over");
    }

    @Test
    public void testPositionInSeq() throws Exception {
        PositionInSeq r = PositionInSeq.fromFile(SRC_DIR + "position_in_seq.bin");

        ArrayList<Integer> expected = new ArrayList<Integer>(3);
        expected.add(1);
        expected.add(2);
        expected.add(3);

        assertEquals(r.numbers(), expected);
    }

    @Test
    public void testProcessRotate() throws Exception {
        ProcessRotate r = ProcessRotate.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1(), "Hello".getBytes());
        assertEquals(r.buf2(), "World".getBytes());
        assertEquals(r.buf3(), "There".getBytes());
    }

    @Test
    public void testStrEos() throws Exception {
        StrEos r = StrEos.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.str(), "foo|bar|baz@");
    }

    @Test
    public void testProcessToUser() throws Exception {
        ProcessToUser r = ProcessToUser.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1().str(), "Hello");
    }
}

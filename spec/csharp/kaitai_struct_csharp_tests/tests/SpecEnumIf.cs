using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumIf : CommonSpec
    {
        [Test]
        public void TestEnumIf()
        {
            EnumIf r = EnumIf.FromFile(SourceFile("if_struct.bin"));

            Assert.AreEqual(r.Op1.Opcode, EnumIf.Opcodes.AString);
            Assert.AreEqual(r.Op1.ArgStr.Str, "foo");

            Assert.AreEqual(r.Op2.Opcode, EnumIf.Opcodes.ATuple);
            Assert.AreEqual(r.Op2.ArgTuple.Num1, 0x42);
            Assert.AreEqual(r.Op2.ArgTuple.Num2, 0x43);

            Assert.AreEqual(r.Op3.Opcode, EnumIf.Opcodes.AString);
            Assert.AreEqual(r.Op3.ArgStr.Str, "bar");
        }
    }
}
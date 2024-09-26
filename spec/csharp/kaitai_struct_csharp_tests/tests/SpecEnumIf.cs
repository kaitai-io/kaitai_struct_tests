// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumIf : CommonSpec
    {
        [Test]
        public void TestEnumIf()
        {
            var r = EnumIf.FromFile(SourceFile("if_struct.bin"));

            Assert.AreEqual(EnumIf.Opcodes.AString, r.Op1.Opcode);
            Assert.AreEqual("foo", r.Op1.ArgStr.Str);
            Assert.AreEqual(EnumIf.Opcodes.ATuple, r.Op2.Opcode);
            Assert.AreEqual(66, r.Op2.ArgTuple.Num1);
            Assert.AreEqual(67, r.Op2.ArgTuple.Num2);
            Assert.AreEqual(EnumIf.Opcodes.AString, r.Op3.Opcode);
            Assert.AreEqual("bar", r.Op3.ArgStr.Str);
        }
    }
}

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIfStruct : CommonSpec
    {
        [Test]
        public void TestIfStruct()
        {
            IfStruct r = IfStruct.FromFile(SourceFile("if_struct.bin"));

            Assert.AreEqual(r.Op1.Opcode, 0x53);
            Assert.AreEqual(r.Op1.ArgStr.Str, "foo");

            Assert.AreEqual(r.Op2.Opcode, 0x54);
            Assert.AreEqual(r.Op2.ArgTuple.Num1, 0x42);
            Assert.AreEqual(r.Op2.ArgTuple.Num2, 0x43);

            Assert.AreEqual(r.Op3.Opcode, 0x53);
            Assert.AreEqual(r.Op3.ArgStr.Str, "bar");
        }
    }
}
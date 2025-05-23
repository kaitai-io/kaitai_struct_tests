// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIfStruct : CommonSpec
    {
        [Test]
        public void TestIfStruct()
        {
            var r = IfStruct.FromFile(SourceFile("if_struct.bin"));
            Assert.AreEqual(r.Op1.Opcode, 83);
            Assert.IsNull(r.Op1.ArgTuple);
            Assert.AreEqual(r.Op1.ArgStr.Str, "foo");
            Assert.AreEqual(r.Op2.Opcode, 84);
            Assert.AreEqual(r.Op2.ArgTuple.Num1, 66);
            Assert.AreEqual(r.Op2.ArgTuple.Num2, 67);
            Assert.IsNull(r.Op2.ArgStr);
            Assert.AreEqual(r.Op3.Opcode, 83);
            Assert.IsNull(r.Op3.ArgTuple);
            Assert.AreEqual(r.Op3.ArgStr.Str, "bar");
        }
    }
}

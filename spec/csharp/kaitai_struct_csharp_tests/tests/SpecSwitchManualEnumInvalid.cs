using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualEnumInvalid : CommonSpec
    {
        [Test]
        public void TestSwitchManualEnumInvalid()
        {
            var r = SwitchManualEnumInvalid.FromFile(SourceFile("enum_negative.bin"));


            Assert.AreEqual(r.Opcodes.Count, 2);
            Assert.AreEqual(r.Opcodes[0].Code, (SwitchManualEnumInvalid.Opcode.CodeEnum) 255);
            Assert.IsNull(r.Opcodes[0].Body);
            Assert.AreEqual(r.Opcodes[1].Code, (SwitchManualEnumInvalid.Opcode.CodeEnum) 1);
            Assert.IsNull(r.Opcodes[1].Body);
        }
    }
}

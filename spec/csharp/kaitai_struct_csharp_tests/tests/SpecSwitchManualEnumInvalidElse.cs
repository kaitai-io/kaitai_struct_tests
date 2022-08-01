using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualEnumInvalidElse : CommonSpec
    {
        [Test]
        public void TestSwitchManualEnumInvalidElse()
        {
            var r = SwitchManualEnumInvalidElse.FromFile(SourceFile("enum_negative.bin"));


            Assert.AreEqual(r.Opcodes.Count, 2);
            Assert.AreEqual(r.Opcodes[0].Code, (SwitchManualEnumInvalidElse.Opcode.CodeEnum) 255);
            Assert.AreEqual(((SwitchManualEnumInvalidElse.Opcode.Defval) (r.Opcodes[0].Body)).Value, 123);
            Assert.AreEqual(r.Opcodes[1].Code, (SwitchManualEnumInvalidElse.Opcode.CodeEnum) 1);
            Assert.AreEqual(((SwitchManualEnumInvalidElse.Opcode.Defval) (r.Opcodes[1].Body)).Value, 123);
        }
    }
}

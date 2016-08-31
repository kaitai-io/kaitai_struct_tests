using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualEnum : CommonSpec
    {
        [Test]
        public void TestSwitchManualEnum()
        {
            var r = SwitchManualEnum.FromFile(SourceFile("switch_opcodes.bin"));
            Assert.AreEqual(r.Opcodes.Count, 4);
    
            Assert.AreEqual(r.Opcodes[0].Code, 83);
            Assert.AreEqual(((Kaitai.SwitchManualInt.Opcode.Strval)r.Opcodes[0].Body).Value, "foobar");

            Assert.AreEqual(r.Opcodes[1].Code, 73);
            Assert.AreEqual(((Kaitai.SwitchManualInt.Opcode.Intval)r.Opcodes[1].Body).Value, 0x42);

            Assert.AreEqual(r.Opcodes[2].Code, 73);
            Assert.AreEqual(((Kaitai.SwitchManualInt.Opcode.Intval)r.Opcodes[2].Body).Value, 0x37);

            Assert.AreEqual(r.Opcodes[3].Code, 83);
            Assert.AreEqual(((Kaitai.SwitchManualInt.Opcode.Strval)r.Opcodes[3].Body).Value, "");
        }
    }
}

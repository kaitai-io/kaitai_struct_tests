using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualStr : CommonSpec
    {
        [Test]
        public void TestSwitchManualStr()
        {
            var r = SwitchManualStr.FromFile(SourceFile("switch_opcodes.bin"));
            Assert.AreEqual(r.Opcodes.Count, 4);

            Assert.AreEqual(r.Opcodes[0].Code, "S");
            Assert.AreEqual(((SwitchManualStr.Opcode.Strval)r.Opcodes[0].Body).Value, "foobar");

            Assert.AreEqual(r.Opcodes[1].Code, "I");
            Assert.AreEqual(((SwitchManualStr.Opcode.Intval)r.Opcodes[1].Body).Value, 0x42);

            Assert.AreEqual(r.Opcodes[2].Code, "I");
            Assert.AreEqual(((SwitchManualStr.Opcode.Intval)r.Opcodes[2].Body).Value, 0x37);

            Assert.AreEqual(r.Opcodes[3].Code, "S");
            Assert.AreEqual(((SwitchManualStr.Opcode.Strval)r.Opcodes[3].Body).Value, "");
        }
    }
}

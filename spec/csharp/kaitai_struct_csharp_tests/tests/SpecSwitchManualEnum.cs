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
    
            Assert.AreEqual(r.Opcodes[0].Code, SwitchManualEnum.Opcode.CodeEnum.Strval);
            Assert.AreEqual(((SwitchManualEnum.Opcode.Strval)r.Opcodes[0].Body).Value, "foobar");

            Assert.AreEqual(r.Opcodes[1].Code, SwitchManualEnum.Opcode.CodeEnum.Intval);
            Assert.AreEqual(((SwitchManualEnum.Opcode.Intval)r.Opcodes[1].Body).Value, 0x42);

            Assert.AreEqual(r.Opcodes[2].Code, SwitchManualEnum.Opcode.CodeEnum.Intval);
            Assert.AreEqual(((SwitchManualEnum.Opcode.Intval)r.Opcodes[2].Body).Value, 0x37);

            Assert.AreEqual(r.Opcodes[3].Code, Kaitai.SwitchManualEnum.Opcode.CodeEnum.Strval);
            Assert.AreEqual(((SwitchManualEnum.Opcode.Strval)r.Opcodes[3].Body).Value, "");
        }
    }
}

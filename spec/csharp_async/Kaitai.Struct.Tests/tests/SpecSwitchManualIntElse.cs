// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualIntElse : CommonSpec
    {
        [Test]
        public async Task TestSwitchManualIntElse()
        {
            var r = SwitchManualIntElse.FromFile(SourceFile("switch_opcodes2.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Opcodes.Count, 4);
            Assert.AreEqual(r.Opcodes[0].Code, 83);
            Assert.AreEqual(((SwitchManualIntElse.Opcode.Strval) (r.Opcodes[0].Body)).Value, "foo");
            Assert.AreEqual(r.Opcodes[1].Code, 88);
            Assert.AreEqual(((SwitchManualIntElse.Opcode.Noneval) (r.Opcodes[1].Body)).Filler, 66);
            Assert.AreEqual(r.Opcodes[2].Code, 89);
            Assert.AreEqual(((SwitchManualIntElse.Opcode.Noneval) (r.Opcodes[2].Body)).Filler, 51966);
            Assert.AreEqual(r.Opcodes[3].Code, 73);
            Assert.AreEqual(((SwitchManualIntElse.Opcode.Intval) (r.Opcodes[3].Body)).Value, 7);
        }
    }
}

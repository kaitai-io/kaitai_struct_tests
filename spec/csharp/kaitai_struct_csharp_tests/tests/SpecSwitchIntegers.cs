// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchIntegers : CommonSpec
    {
        [Test]
        public void TestSwitchIntegers()
        {
            var r = SwitchIntegers.FromFile(SourceFile("switch_integers.bin"));

            Assert.AreEqual(4, r.Opcodes.Count);
            Assert.AreEqual(1, r.Opcodes[0].Code);
            Assert.AreEqual(7, r.Opcodes[0].Body);
            Assert.AreEqual(2, r.Opcodes[1].Code);
            Assert.AreEqual(16448, r.Opcodes[1].Body);
            Assert.AreEqual(4, r.Opcodes[2].Code);
            Assert.AreEqual(4919, r.Opcodes[2].Body);
            Assert.AreEqual(8, r.Opcodes[3].Code);
            Assert.AreEqual(4919, r.Opcodes[3].Body);
        }
    }
}

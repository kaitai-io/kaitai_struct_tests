using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchMultiBoolOps : CommonSpec
    {
        [Test]
        public void TestSwitchMultiBoolOps()
        {
            var r = SwitchMultiBoolOps.FromFile(SourceFile("switch_integers.bin"));
            Assert.AreEqual(r.Opcodes.Count, 4);
    
            Assert.AreEqual(r.Opcodes[0].Code, 1);
            Assert.AreEqual(r.Opcodes[0].Body, 7);
    
            Assert.AreEqual(r.Opcodes[1].Code, 2);
            Assert.AreEqual(r.Opcodes[1].Body, 0x4040);
    
            Assert.AreEqual(r.Opcodes[2].Code, 4);
            Assert.AreEqual(r.Opcodes[2].Body, 4919);
    
            Assert.AreEqual(r.Opcodes[3].Code, 8);
            Assert.AreEqual(r.Opcodes[3].Body, 4919);
        }
    }
}

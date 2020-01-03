using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchRepeatExpr : CommonSpec
    {
        [Test]
        public void TestSwitchRepeatExpr()
        {
            var r = SwitchRepeatExpr.FromFile(SourceFile("switch_tlv.bin"));
            Assert.AreEqual(r.Code, 0x11);
            Assert.AreEqual(r.Size, 9);
            Assert.AreEqual(((SwitchRepeatExpr.One) r.Body[0]).First, System.Text.Encoding.ASCII.GetBytes("Stuff\0Me\0"));
        }
    }
}

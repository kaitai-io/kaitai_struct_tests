using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNavParentVsValueInst : CommonSpec
    {
        [Test]
        public void TestNavParentVsValueInst()
        {
            var r = NavParentVsValueInst.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(r.S1, "foo");
        }
    }
}

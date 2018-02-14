using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecJsSignedRightShift : CommonSpec
    {
        [Test]
        public void TestJsSignedRightShift()
        {
            var r = JsSignedRightShift.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.ShouldBe40000000, 0x40000000);
            Assert.AreEqual(r.ShouldBeA00000, 0xa00000);
        }
    }
}

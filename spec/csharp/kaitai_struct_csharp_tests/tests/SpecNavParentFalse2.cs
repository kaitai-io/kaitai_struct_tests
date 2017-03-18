using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNavParentFalse2 : CommonSpec
    {
        [Test]
        public void TestNavParentFalse2()
        {
            var r = NavParentFalse2.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Parentless.Foo, 80);
        }
    }
}

// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNavParentOverride : CommonSpec
    {
        [Test]
        public void TestNavParentOverride()
        {
            var r = NavParentOverride.FromFile(SourceFile("nav_parent_codes.bin"));

            Assert.AreEqual(3, r.ChildSize);
            Assert.AreEqual(new byte[] { 73, 49, 50 }, r.Child1.Data);
            Assert.AreEqual(new byte[] { 51, 66, 98 }, r.Mediator2.Child2.Data);
        }
    }
}

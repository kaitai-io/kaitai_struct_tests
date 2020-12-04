// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNavRootRecursive : CommonSpec
    {
        [Test]
        public void TestNavRootRecursive()
        {
            var r = NavRootRecursive.FromFile(SourceFile("enum_negative.bin"));

            Assert.AreEqual(r.Value, 255);
            Assert.AreEqual(r.Next.Value, 1);
            Assert.AreEqual(r.Next.RootValue, 255);
            Assert.IsNull(r.Next.Next);
        }
    }
}
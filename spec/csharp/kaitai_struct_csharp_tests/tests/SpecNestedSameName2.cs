using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNestedSameName2 : CommonSpec
    {
        [Test]
        public void TestNestedSameName2()
        {
            var r = NestedSameName2.FromFile(SourceFile("nested_same_name2.bin"));
            Assert.AreEqual(r.Version, 0x42);
            Assert.AreEqual(r.MainData.MainSize, 2);
            Assert.AreEqual(r.MainData.Foo.Data1, new byte[] { 0x11, 0x11, 0x11, 0x11 });
            Assert.AreEqual(r.Dummy.DummySize, 3);
            Assert.AreEqual(r.Dummy.Foo.Data2, new byte[] { 0x22, 0x22, 0x22, 0x22, 0x22, 0x22 });
        }
    }
}

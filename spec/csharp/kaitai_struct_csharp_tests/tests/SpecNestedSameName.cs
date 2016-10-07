using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNestedSameName : CommonSpec
    {
        [Test]
        public void TestNestedSameName()
        {
            var r = NestedSameName.FromFile(SourceFile("repeat_n_struct.bin"));
            Assert.AreEqual(r.MainData.MainSize, 2);
            Assert.AreEqual(r.MainData.Foo.Data, new byte[] { 0x10, 0, 0, 0 });
        }
    }
}

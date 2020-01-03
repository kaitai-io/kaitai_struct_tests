// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNestedSameName2 : CommonSpec
    {
        [Test]
        public async Task TestNestedSameName2()
        {
            var r = NestedSameName2.FromFile(SourceFile("nested_same_name2.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Version, 66);
            Assert.AreEqual(r.MainData.MainSize, 2);
            Assert.AreEqual(r.MainData.Foo.Data1, new byte[] { 17, 17, 17, 17 });
            Assert.AreEqual(r.Dummy.DummySize, 3);
            Assert.AreEqual(r.Dummy.Foo.Data2, new byte[] { 34, 34, 34, 34, 34, 34 });
        }
    }
}

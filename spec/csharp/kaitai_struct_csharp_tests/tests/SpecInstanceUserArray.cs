using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecInstanceUserArray : CommonSpec
    {
        [Test]
        public void TestInstanceUserArray()
        {
            InstanceUserArray r = InstanceUserArray.FromFile(SourceFile("instance_std_array.bin"));

            Assert.AreEqual(r.Ofs, 0x10);
            Assert.AreEqual(r.QtyEntries, 3);
            Assert.AreEqual(r.EntrySize, 4);

            Assert.AreEqual(r.UserEntries.Count, 3);
            Assert.AreEqual(r.UserEntries[0].Word1, 0x1111);
            Assert.AreEqual(r.UserEntries[0].Word2, 0x1111);
            Assert.AreEqual(r.UserEntries[1].Word1, 0x2222);
            Assert.AreEqual(r.UserEntries[1].Word2, 0x2222);
            Assert.AreEqual(r.UserEntries[2].Word1, 0x3333);
            Assert.AreEqual(r.UserEntries[2].Word2, 0x3333);
        }
    }
}
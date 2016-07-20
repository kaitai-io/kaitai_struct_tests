using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecInstanceStdArray : CommonSpec
    {
        [Test]
        public void TestInstanceStdArray()
        {
            InstanceStdArray r = InstanceStdArray.FromFile(SourceFile("instance_std_array.bin"));

            Assert.AreEqual(r.Ofs, 0x10);
            Assert.AreEqual(r.QtyEntries, 3);
            Assert.AreEqual(r.EntrySize, 4);

            Assert.AreEqual(r.Entries.Count, 3);
            Assert.AreEqual(r.Entries[0], new byte[] {0x11, 0x11, 0x11, 0x11});
            Assert.AreEqual(r.Entries[1], new byte[] {0x22, 0x22, 0x22, 0x22});
            Assert.AreEqual(r.Entries[2], new byte[] {0x33, 0x33, 0x33, 0x33});
        }
    }
}
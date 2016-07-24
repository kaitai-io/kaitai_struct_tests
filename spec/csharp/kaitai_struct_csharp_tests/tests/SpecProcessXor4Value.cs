using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessXor4Value : CommonSpec
    {
        [Test]
        public void TestProcessXor4Value()
        {
            ProcessXor4Value r = ProcessXor4Value.FromFile(SourceFile("process_xor_4.bin"));

            CollectionAssert.AreEqual(r.Key, new byte[] {0xec, 0xbb, 0xa3, 0x14});
            Assert.AreEqual(System.Text.Encoding.UTF8.GetString(r.Buf), "foo bar");
        }
    }
}
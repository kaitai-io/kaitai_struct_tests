using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessXorConst : CommonSpec
    {
        [Test]
        public void TestProcessXorConst()
        {
            ProcessXorConst r = ProcessXorConst.FromFile(SourceFile("process_xor_1.bin"));

            Assert.AreEqual(r.Key, 0xff);
            Assert.AreEqual(System.Text.Encoding.UTF8.GetString(r.Buf), "foo bar");
        }
    }
}
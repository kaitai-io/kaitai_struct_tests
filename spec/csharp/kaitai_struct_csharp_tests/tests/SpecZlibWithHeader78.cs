using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecZlibWithHeader78 : CommonSpec
    {
        [Test]
        public void TestZlibWithHeader78()
        {
            ZlibWithHeader78 r = ZlibWithHeader78.FromFile(SourceFile("zlib_with_header_78.bin"));
            
            Assert.AreEqual(System.Text.Encoding.UTF8.GetString(r.Data), "a quick brown fox jumps over");
        }
    }
}
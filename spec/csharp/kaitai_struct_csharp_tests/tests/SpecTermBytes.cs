using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTermBytes : CommonSpec
    {
        [Test]
        public void TestTermBytes()
        {
            var r = TermBytes.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(r.S1, new byte[] { 0x66, 0x6f, 0x6f });                                                                                    
            Assert.AreEqual(r.S2, new byte[] { 0x62, 0x61, 0x72 });                                                                                    
            Assert.AreEqual(r.S3, new byte[] { 0x7c, 0x62, 0x61, 0x7a, 0x40 });                                                                        
        }
    }
}

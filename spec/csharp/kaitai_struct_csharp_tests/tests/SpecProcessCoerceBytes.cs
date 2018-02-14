using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessCoerceBytes : CommonSpec
    {
        [Test]
        public void TestProcessCoerceBytes()
        {
            var r = ProcessCoerceBytes.FromFile(SourceFile("process_coerce_bytes.bin"));
            Assert.AreEqual(r.Records[0].Flag, 0);
            Assert.AreEqual(r.Records[0].Buf, "AAAA");
            Assert.AreEqual(r.Records[1].Flag, 1);
            Assert.AreEqual(r.Records[1].Buf, "BBBB");
        }
    }
}

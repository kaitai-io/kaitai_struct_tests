using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessCoerceUsertype2 : CommonSpec
    {
        [Test]
        public void TestProcessCoerceUsertype2()
        {
            var r = ProcessCoerceUsertype2.FromFile(SourceFile("process_coerce_bytes.bin"));
            Assert.AreEqual(r.Records[0].Flag, 0);
            Assert.AreEqual(r.Records[0].Buf.Value, 0x41414141);
            Assert.AreEqual(r.Records[1].Flag, 1);
            Assert.AreEqual(r.Records[1].Buf.Value, 0x42424242);
        }
    }
}

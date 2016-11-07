using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprIoPos : CommonSpec
    {
        [Test]
        public void TestExprIoPos()
        {
            var r = ExprIoPos.FromFile(SourceFile("expr_io_pos.bin"));
            Assert.AreEqual(r.Substream1.MyStr, "CURIOSITY");
            Assert.AreEqual(r.Substream1.Body, new byte[] { 0x11, 0x22, 0x33, 0x44 });
            Assert.AreEqual(r.Substream1.Number, 0x42);
    
            Assert.AreEqual(r.Substream2.MyStr, "KILLED");
            Assert.AreEqual(r.Substream2.Body, new byte[] { 0x61, 0x20, 0x63, 0x61, 0x74 });
            Assert.AreEqual(r.Substream2.Number, 0x67);
        }
    }
}

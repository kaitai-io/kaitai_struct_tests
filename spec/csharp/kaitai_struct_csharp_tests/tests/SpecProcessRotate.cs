using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessRotate : CommonSpec
    {
        [Test]
        public void TestProcessRotate()
        {
            ProcessRotate r = ProcessRotate.FromFile(SourceFile("process_rotate.bin"));

            Assert.AreEqual(r.Buf1, System.Text.Encoding.ASCII.GetBytes("Hello"));
            Assert.AreEqual(r.Buf2, System.Text.Encoding.ASCII.GetBytes("World"));
            Assert.AreEqual(r.Buf3, System.Text.Encoding.ASCII.GetBytes("There"));
        }
    }
}
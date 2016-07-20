using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessXorValue : CommonSpec
    {
        [Test]
        public void TestProcessXorValue()
        {
            ProcessXorValue r = ProcessXorValue.FromFile(SourceFile("process_xor_1.bin"));

            Assert.AreEqual(r.Key, 0xff);
            Assert.AreEqual(System.Text.Encoding.UTF8.GetString(r.Buf), "foo bar");
        }
    }
}
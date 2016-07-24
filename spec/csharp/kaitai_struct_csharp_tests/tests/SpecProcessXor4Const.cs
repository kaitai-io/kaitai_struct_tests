using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessXor4Const : CommonSpec
    {
        [Test]
        public void TestProcessXor4Const()
        {
            ProcessXor4Const r = ProcessXor4Const.FromFile(SourceFile("process_xor_4.bin"));

            Assert.AreEqual(System.Text.Encoding.UTF8.GetString(r.Buf), "foo bar");
        }
    }
}
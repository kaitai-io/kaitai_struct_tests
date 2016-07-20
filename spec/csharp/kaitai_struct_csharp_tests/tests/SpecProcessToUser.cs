using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessToUser : CommonSpec
    {
        [Test]
        public void TestProcessToUser()
        {
            ProcessToUser r = ProcessToUser.FromFile(SourceFile("process_rotate.bin"));

            Assert.AreEqual(r.Buf1.Str, "Hello");
        }
    }
}
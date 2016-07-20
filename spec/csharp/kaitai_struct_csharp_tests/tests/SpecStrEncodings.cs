using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodings : CommonSpec
    {
        [Test]
        public void TestStrEncodings()
        {
            StrEncodings r = StrEncodings.FromFile(SourceFile("str_encodings.bin"));

            Assert.AreEqual(r.Str1, "Some ASCII");
            Assert.AreEqual(r.Str2, "こんにちは");
            Assert.AreEqual(r.Str3, "こんにちは");
            Assert.AreEqual(r.Str4, "░▒▓");
        }
    }
}
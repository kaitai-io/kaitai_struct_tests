using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodingsDefault : CommonSpec
    {
        [Test]
        public void TestStrEncodingsDefault()
        {
            var r = StrEncodingsDefault.FromFile(SourceFile("str_encodings.bin"));
            Assert.AreEqual(r.Str1, "Some ASCII");
            Assert.AreEqual(r.Rest.Str2, "こんにちは");
            Assert.AreEqual(r.Rest.Str3, "こんにちは");
            Assert.AreEqual(r.Rest.Str4, "░▒▓");
        }
    }
}

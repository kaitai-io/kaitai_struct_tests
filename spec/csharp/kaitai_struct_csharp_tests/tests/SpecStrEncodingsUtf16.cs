// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodingsUtf16 : CommonSpec
    {
        [Test]
        public void TestStrEncodingsUtf16()
        {
            var r = StrEncodingsUtf16.FromFile(SourceFile("str_encodings_utf16.bin"));

            Assert.AreEqual(r.LenBe, 12);
            Assert.AreEqual(r.BeBomRemoved.Bom, 65279);
            Assert.AreEqual(r.BeBomRemoved.Str, "\u3053\u3093\u306b\u3061\u306f");
            Assert.AreEqual(r.LenLe, 12);
            Assert.AreEqual(r.LeBomRemoved.Bom, 65279);
            Assert.AreEqual(r.LeBomRemoved.Str, "\u3053\u3093\u306b\u3061\u306f");
        }
    }
}

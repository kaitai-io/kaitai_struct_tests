using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodingsEscapingEnc : CommonSpec
    {
        [Test]
        public void TestStrEncodingsEscapingEnc()
        {
            var r = StrEncodingsEscapingEnc.FromFile(SourceFile("str_encodings.bin"));

            AssertUnknownEncoding("ASCII\\\\x",
                delegate
                {
                    _ = r.Str1.V;
                }
            );
            AssertUnknownEncoding("UTF-8\\'x",
                delegate
                {
                    _ = r.Str2.V;
                }
            );
            AssertUnknownEncoding("SJIS\\\"x",
                delegate
                {
                    _ = r.Str3.V;
                }
            );
            AssertUnknownEncoding("IBM437\\nx",
                delegate
                {
                    _ = r.Str4.V;
                }
            );
        }

        private void AssertUnknownEncoding(string expectedEncoding, TestDelegate code)
        {
            var exc = Assert.Throws<System.ArgumentException>(code);
            StringAssert.StartsWith("'" + expectedEncoding + "' is not a supported encoding name.", exc.Message);
        }
    }
}

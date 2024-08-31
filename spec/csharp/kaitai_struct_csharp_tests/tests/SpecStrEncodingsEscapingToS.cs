using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodingsEscapingToS : CommonSpec
    {
        [Test]
        public void TestStrEncodingsEscapingToS()
        {
            var r = StrEncodingsEscapingToS.FromFile(SourceFile("str_encodings.bin"));

            AssertUnknownEncoding("ASCII\\\\x",
                delegate
                {
                    _ = r.Str1;
                }
            );
            AssertUnknownEncoding("UTF-8\\'x",
                delegate
                {
                    _ = r.Str2;
                }
            );
            AssertUnknownEncoding("SJIS\\\"x",
                delegate
                {
                    _ = r.Str3;
                }
            );
            AssertUnknownEncoding("IBM437\\nx",
                delegate
                {
                    _ = r.Str4;
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

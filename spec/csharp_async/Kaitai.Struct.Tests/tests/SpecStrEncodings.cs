using System.Text;
using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodings : CommonSpec
    {
        [Test]
        public async Task TestStrEncodings()
        {
            Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);

            var r = StrEncodings.FromFile(SourceFile("str_encodings.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Str1, "Some ASCII");
            Assert.AreEqual(r.Str2, "\u3053\u3093\u306b\u3061\u306f");
            Assert.AreEqual(r.Str3, "\u3053\u3093\u306b\u3061\u306f");
            Assert.AreEqual(r.Str4, "\u2591\u2592\u2593");
        }
    }
}

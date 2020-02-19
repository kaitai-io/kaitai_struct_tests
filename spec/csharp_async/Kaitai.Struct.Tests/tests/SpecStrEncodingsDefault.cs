using System.Text;
using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEncodingsDefault : CommonSpec
    {
        [Test]
        public async Task TestStrEncodingsDefault()
        {
            Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);

            var r = StrEncodingsDefault.FromFile(SourceFile("str_encodings.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Str1, "Some ASCII");
            Assert.AreEqual(r.Rest.Str2, "\u3053\u3093\u306b\u3061\u306f");
            Assert.AreEqual(r.Rest.Str3, "\u3053\u3093\u306b\u3061\u306f");
            Assert.AreEqual(r.Rest.Str4, "\u2591\u2592\u2593");
        }
    }
}

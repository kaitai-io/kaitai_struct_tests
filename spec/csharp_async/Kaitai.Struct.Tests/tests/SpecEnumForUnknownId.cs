using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumForUnknownId : CommonSpec
    {
        [Test]
        public async Task TestEnumForUnknownId()
        {
            var r = EnumForUnknownId.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.One, (Kaitai.EnumForUnknownId.Animal) 80);
        }
    }
}

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatEosU4 : CommonSpec
    {
        [Test]
        public async Task TestRepeatEosU4()
        {
            RepeatEosU4 r = RepeatEosU4.FromFile(SourceFile("repeat_eos_struct.bin"));
            await r.ReadAsync();

            CollectionAssert.AreEqual(r.Numbers, new long[] {0, 0x42, 0x42, 0x815});
        }
    }
}
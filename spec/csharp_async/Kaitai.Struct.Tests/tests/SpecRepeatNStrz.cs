using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatNStrz : CommonSpec
    {
        [Test]
        public async Task TestRepeatNStrz()
        {
            RepeatNStrz r = RepeatNStrz.FromFile(SourceFile("repeat_n_strz.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Qty, 2);
            CollectionAssert.AreEqual(r.Lines, new [] {"foo", "bar"});
        }
    }
}
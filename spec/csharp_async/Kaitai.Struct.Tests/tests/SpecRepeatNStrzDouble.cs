using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatNStrzDouble : CommonSpec
    {
        [Test]
        public async Task TestRepeatNStrzDouble()
        {
            var r = RepeatNStrzDouble.FromFile(SourceFile("repeat_n_strz.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Qty, 2);
            CollectionAssert.AreEqual(r.Lines1, new [] {"foo"});
            CollectionAssert.AreEqual(r.Lines2, new [] {"bar"});
        }
    }
}

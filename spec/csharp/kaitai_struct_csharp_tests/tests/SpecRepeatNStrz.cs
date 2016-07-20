using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatNStrz : CommonSpec
    {
        [Test]
        public void TestRepeatNStrz()
        {
            RepeatNStrz r = RepeatNStrz.FromFile(SourceFile("repeat_n_strz.bin"));

            Assert.AreEqual(r.Qty, 2);
            CollectionAssert.AreEqual(r.Lines, new [] {"foo", "bar"});
        }
    }
}
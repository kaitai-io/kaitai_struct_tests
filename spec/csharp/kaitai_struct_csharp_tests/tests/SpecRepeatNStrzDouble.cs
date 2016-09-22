using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatNStrzDouble : CommonSpec
    {
        [Test]
        public void TestRepeatNStrzDouble()
        {
            var r = RepeatNStrzDouble.FromFile(SourceFile("repeat_n_strz.bin"));
            Assert.AreEqual(r.Qty, 2);
            CollectionAssert.AreEqual(r.Lines1, new [] {"foo"});
            CollectionAssert.AreEqual(r.Lines2, new [] {"bar"});
        }
    }
}

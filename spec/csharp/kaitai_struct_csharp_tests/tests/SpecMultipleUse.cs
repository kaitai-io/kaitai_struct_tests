// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecMultipleUse : CommonSpec
    {
        [Test]
        public void TestMultipleUse()
        {
            var r = MultipleUse.FromFile(SourceFile("position_abs.bin"));

            Assert.AreEqual(32, r.T1.FirstUse.Value);
            Assert.AreEqual(32, r.T2.SecondUse.Value);
        }
    }
}

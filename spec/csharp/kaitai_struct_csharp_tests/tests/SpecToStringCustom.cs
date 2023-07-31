using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecToStringCustom : CommonSpec
    {
        [Test]
        public void TestToStringCustom()
        {
            var r = ToStringCustom.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.ToString(), "s1 = foo, s2 = bar");
        }
    }
}

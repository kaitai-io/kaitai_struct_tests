using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImportsAbs : CommonSpec
    {
        [Test]
        public void TestImportsAbs()
        {
            var r = ImportsAbs.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Len.Value, 80);
            Assert.AreEqual(r.Body.Length, 80);
        }
    }
}

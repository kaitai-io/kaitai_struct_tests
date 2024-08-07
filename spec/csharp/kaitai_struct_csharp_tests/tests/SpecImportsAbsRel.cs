// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImportsAbsRel : CommonSpec
    {
        [Test]
        public void TestImportsAbsRel()
        {
            var r = ImportsAbsRel.FromFile(SourceFile("fixed_struct.bin"));

            Assert.AreEqual(r.One, 80);
            Assert.AreEqual(r.Two.One, 65);
            Assert.AreEqual(r.Two.Two.One, 67);
        }
    }
}

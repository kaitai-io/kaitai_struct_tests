// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprSizeofType0 : CommonSpec
    {
        [Test]
        public void TestExprSizeofType0()
        {
            var r = ExprSizeofType0.FromFile(SourceFile("fixed_struct.bin"));

            Assert.AreEqual((1 + 4) + 2, r.SizeofBlock);
        }
    }
}

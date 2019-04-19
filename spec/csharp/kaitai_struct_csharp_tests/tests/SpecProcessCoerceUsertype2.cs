// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessCoerceUsertype2 : CommonSpec
    {
        [Test]
        public void TestProcessCoerceUsertype2()
        {
            var r = ProcessCoerceUsertype2.FromFile(SourceFile("process_coerce_bytes.bin"));

            Assert.AreEqual(r.Records[0].Flag, 0);
            Assert.AreEqual(r.Records[0].Buf.Value, 1094795585);
            Assert.AreEqual(r.Records[1].Flag, 1);
            Assert.AreEqual(r.Records[1].Buf.Value, 1111638594);
        }
    }
}

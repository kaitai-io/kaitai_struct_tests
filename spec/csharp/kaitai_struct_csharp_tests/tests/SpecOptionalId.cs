using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOptionalId : CommonSpec
    {
        [Test]
        public void TestOptionalId()
        {
            var r = OptionalId.FromFile(SourceFile("fixed_struct.bin"));

            AreEqual(r.Unnamed_0, 80);
            AreEqual(r.Unnamed_1, 65);
            AreEqual(r.Unnamed_2, new byte[] { 67, 75, 45, 49, 255 });
        }
    }
}

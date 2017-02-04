using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRecursiveOne : CommonSpec
    {
        [Test]
        public void TestRecursiveOne()
        {
            var r = RecursiveOne.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.One, 0x50);
            var rec1 = (RecursiveOne) r.Next;
            Assert.AreEqual(rec1.One, 0x41);
            var rec2 = (RecursiveOne) rec1.Next;
            Assert.AreEqual(rec2.One, 0x43);
            var rec3 = (RecursiveOne.Fini) rec2.Next;
            Assert.AreEqual(rec3.Finisher, 0x2d4b);
        }
    }
}

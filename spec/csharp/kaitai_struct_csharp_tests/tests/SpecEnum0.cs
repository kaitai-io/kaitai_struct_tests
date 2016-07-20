using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnum0 : CommonSpec
    {
        [Test]
        public void TestEnum0()
        {
            Enum0 r = Enum0.FromFile(SourceFile("enum_0.bin"));

            Assert.AreEqual(r.Pet1, Enum0.Animal.Cat);
            Assert.AreEqual(r.Pet2, Enum0.Animal.Chicken);
        }
    }
}
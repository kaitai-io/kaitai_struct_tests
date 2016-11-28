using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprEnum : CommonSpec
    {
        [Test]
        public void TestExprEnum()
        {
            var r = ExprEnum.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(ExprEnum.Animal.Dog, r.ConstDog);
            Assert.AreEqual(ExprEnum.Animal.Boom, r.DerivedBoom);
            Assert.AreEqual(ExprEnum.Animal.Dog, r.DerivedDog);
        }
    }
}

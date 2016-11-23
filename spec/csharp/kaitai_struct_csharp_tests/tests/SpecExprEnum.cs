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
            Assert.AreEqual(r.ConstDog, ExprEnum.Animal.Dog);
            Assert.AreEqual(r.ConstNobody, null);
            Assert.AreEqual(r.DerivedBoom, ExprEnum.Animal.Boom);
            Assert.AreEqual(r.DerivedDog, ExprEnum.Animal.Dog);
        }
    }
}

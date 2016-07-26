using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecFloatingPoints : CommonSpec
    {
        [Test]
        public void TestFloatingPoints()
        {
            var fp = FloatingPoints.FromFile(SourceFile("floating_points.bin"));

            const double delta = 1E-6;

            Assert.AreEqual(1.2345f, fp.SingleValue, delta);
            Assert.AreEqual(1.2345f, fp.SingleValueBe, delta);

            Assert.AreEqual(123.456d, fp.DoubleValue, delta);
            Assert.AreEqual(123.456d, fp.DoubleValueBe, delta);

            Assert.AreEqual(2.2345f, fp.SingleValuePlusInt, delta);
            Assert.AreEqual(1.7345d, fp.SingleValuePlusFloat, delta);
            Assert.AreEqual(123.506d, fp.DoubleValuePlusFloat, delta);
        }
    }
}

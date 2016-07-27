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

            Assert.AreEqual(0.5f, fp.SingleValue);
            Assert.AreEqual(0.5f, fp.SingleValueBe);

            Assert.AreEqual(0.25d, fp.DoubleValue);
            Assert.AreEqual(0.25d, fp.DoubleValueBe);
            
            Assert.AreEqual(1.2345d, fp.ApproximateValue, delta);

            Assert.AreEqual(1.5f, fp.SingleValuePlusInt, delta);
            Assert.AreEqual(1.0d, fp.SingleValuePlusFloat, delta);
            Assert.AreEqual(0.3d, fp.DoubleValuePlusFloat, delta);
        }
    }
}

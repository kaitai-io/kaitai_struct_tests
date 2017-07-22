using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIfInstances : CommonSpec
    {
        [Test]
        public void TestIfInstances()
        {
            var r = IfInstances.FromFile(SourceFile("fixed_struct.bin"));
            Assert.IsNull(r.NeverHappens);
        }
    }
}

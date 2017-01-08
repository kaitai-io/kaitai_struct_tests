using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugEnumName : CommonSpec
    {
        [Test]
        public void TestDebugEnumName()
        {
            // this test is meaningful only for languages that have --debug and do
            // not save enum type info
        }
    }
}

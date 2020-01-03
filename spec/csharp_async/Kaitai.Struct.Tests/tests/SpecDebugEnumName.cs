using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugEnumName : CommonSpec
    {
        [Test]
        public async Task TestDebugEnumName()
        {
            // this test is meaningful only for languages that have --debug and do
            // not save enum type info
        }
    }
}

// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIfInstances : CommonSpec
    {
        [Test]
        public async Task TestIfInstances()
        {
            var r = IfInstances.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();

            Assert.IsNull(r.NeverHappens);
        }
    }
}

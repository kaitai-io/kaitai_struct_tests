using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecValidFailEqInt : CommonSpec
    {
        [Test]
        public async Task TestValidFailEqInt()
        {
          Assert.ThrowsAsync<ValidationNotEqualError>(async () =>
            await ValidFailInst.FromFile(SourceFile("fixed_struct.bin")).ReadAsync());
        }
    }
}

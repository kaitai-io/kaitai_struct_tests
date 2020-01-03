using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
  [TestFixture]
  public class SpecValidFailInst : CommonSpec
  {
    [Test]
    public async Task TestValidFailInst()
    {
      Assert.ThrowsAsync<ValidationNotEqualError>(async () =>
        await ValidFailInst.FromFile(SourceFile("fixed_struct.bin")).ReadAsync());
    }
  }
}
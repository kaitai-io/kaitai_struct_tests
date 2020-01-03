using NUnit.Framework;
using System;
using System.Threading.Tasks;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprException : CommonSpec
    {
        [Test]
        public async Task TestDefaultEndianExprException()
        {
            Assert.ThrowsAsync<ValidationNotEqualError>(async () =>
            await ValidFailInst.FromFile(SourceFile("endian_expr.bin")).ReadAsync());
          // TODO FIXME: catch more specific exception
        }
    }
}

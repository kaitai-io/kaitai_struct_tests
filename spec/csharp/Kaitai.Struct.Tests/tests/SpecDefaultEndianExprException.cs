using NUnit.Framework;
using System;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprException : CommonSpec
    {
        [Test]
        public void TestDefaultEndianExprException()
        {
            Assert.Throws<Exception>(
                delegate {
                    DefaultEndianExprException.FromFile(SourceFile("endian_expr.bin"));
                }
            );
            // FIXME: catch more specific exception
        }
    }
}

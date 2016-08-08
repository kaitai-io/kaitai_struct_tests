using System;
using System.IO;

namespace Kaitai
{
    using NUnit.Framework;

    [TestFixture]
    public class SpecEofExceptionBytes : CommonSpec
    {
        [Test]
        public void TestEofExceptionBytes()
        {
            Assert.Throws<EndOfStreamException>(
                delegate {
                    EofExceptionBytes.FromFile(SourceFile("term_strz.bin"));
                }
            );
        }
    }
}

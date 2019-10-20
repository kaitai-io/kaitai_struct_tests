using System;
using System.IO;

namespace Kaitai
{
    using NUnit.Framework;

    [TestFixture]
    public class SpecEosExceptionBytes : CommonSpec
    {
        [Test]
        public void TestEosExceptionBytes()
        {
            Assert.Throws<EndOfStreamException>(
                delegate {
                    EosExceptionBytes.FromFile(SourceFile("term_strz.bin"));
                }
            );
        }
    }
}

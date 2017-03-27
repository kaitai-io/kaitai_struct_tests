using NUnit.Framework;
using System.Text;

namespace Kaitai
{
    [TestFixture]
    public class SpecBytesPadTerm : CommonSpec
    {
        [Test]
        public void TestBytesPadTerm()
        {
            var r = BytesPadTerm.FromFile(SourceFile("str_pad_term.bin"));
            Assert.AreEqual(r.StrPad, Encoding.UTF8.GetBytes("str1"));
            Assert.AreEqual(r.StrTerm, Encoding.UTF8.GetBytes("str2foo"));
            Assert.AreEqual(r.StrTermAndPad, Encoding.UTF8.GetBytes("str+++3bar+++"));
            Assert.AreEqual(r.StrTermInclude, Encoding.UTF8.GetBytes("str4baz@"));
        }
    }
}

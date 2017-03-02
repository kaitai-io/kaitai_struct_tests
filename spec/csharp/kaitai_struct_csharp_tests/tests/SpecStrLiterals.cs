using NUnit.Framework;
using System.Linq;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrLiterals : CommonSpec
    {
        [Test]
        public void TestStrLiterals()
        {
            var r = StrLiterals.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(
                r.ComplexStr.ToCharArray(),
                GenCharArray(new int[] { 0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787 })
            );
            Assert.AreEqual(
                r.DoubleQuotes.ToCharArray(),
                GenCharArray(new int[] { 34, 34, 34 })
            );
            Assert.AreEqual(
                r.Backslashes.ToCharArray(),
                GenCharArray(new int[] { 92, 92, 92 })
            );
        }

        private static char[] GenCharArray(int[] a)
        {
            return a.Select(ch => (char) ch).ToArray();
        }
    }
}

using NUnit.Framework;
using System.Linq;
using System.Threading.Tasks;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrLiterals : CommonSpec
    {
        [Test]
        public async Task TestStrLiterals()
        {
            var r = StrLiterals.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();

            Assert.AreEqual(
                GenCharArray(new int[] { 0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787 }),
                r.ComplexStr.ToCharArray()
            );
            Assert.AreEqual(
                GenCharArray(new int[] { 34, 34, 34 }),
                r.DoubleQuotes.ToCharArray()
            );
            Assert.AreEqual(
                GenCharArray(new int[] { 92, 92, 92 }),
                r.Backslashes.ToCharArray()
            );
            Assert.AreEqual(
                GenCharArray(new int[] { 0, 50, 50 }),
                r.OctalEatup.ToCharArray()
            );
            Assert.AreEqual(
                GenCharArray(new int[] { 2, 50 }),
                r.OctalEatup2.ToCharArray()
            );
        }

        private static char[] GenCharArray(int[] a)
        {
            return a.Select(ch => (char) ch).ToArray();
        }
    }
}

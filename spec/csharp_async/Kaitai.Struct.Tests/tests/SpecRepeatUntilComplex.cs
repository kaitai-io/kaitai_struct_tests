using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatUntilComplex : CommonSpec
    {
        [Test]
        public async Task TestRepeatUntilComplex()
        {
            var r = RepeatUntilComplex.FromFile(SourceFile("repeat_until_complex.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.First.Count, 3);
            Assert.AreEqual(r.First[0].Count, 4);
            Assert.AreEqual(r.First[0].Values, new int[] { 1, 2, 3, 4 });
            Assert.AreEqual(r.First[1].Count, 2);
            Assert.AreEqual(r.First[1].Values, new int[] { 1, 2 });
            Assert.AreEqual(r.First[2].Count, 0);
            Assert.AreEqual(r.First[2].Values, new int[] {  });
    
            Assert.AreEqual(r.Second.Count, 4);
            Assert.AreEqual(r.Second[0].Count, 6);
            Assert.AreEqual(r.Second[0].Values, new int[] { 1, 2, 3, 4, 5, 6 });
            Assert.AreEqual(r.Second[1].Count, 3);
            Assert.AreEqual(r.Second[1].Values, new int[] { 1, 2, 3 });
            Assert.AreEqual(r.Second[2].Count, 4);
            Assert.AreEqual(r.Second[2].Values, new int[] { 1, 2, 3, 4 });
            Assert.AreEqual(r.Second[3].Count, 0);
            Assert.AreEqual(r.Second[3].Values, new int[] {  });
            
            Assert.AreEqual(r.Third, new int[] { 102, 111, 111, 98, 97, 114, 0 });
        }
    }
}

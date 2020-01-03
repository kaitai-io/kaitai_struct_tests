using NUnit.Framework;
using System;
using System.Threading.Tasks;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchCast : CommonSpec
    {
        [Test]
        public async Task TestSwitchCast()
        {
            var r = SwitchCast.FromFile(SourceFile("switch_opcodes.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.FirstObj.Value, "foobar");
            Assert.AreEqual(r.SecondVal, 0x42);
            Assert.Throws<InvalidCastException>(
                delegate {
                    var tmp = r.ErrCast;
                }
            );
        }
    }
}

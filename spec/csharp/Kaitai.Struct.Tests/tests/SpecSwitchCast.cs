using NUnit.Framework;
using System;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchCast : CommonSpec
    {
        [Test]
        public void TestSwitchCast()
        {
            var r = SwitchCast.FromFile(SourceFile("switch_opcodes.bin"));
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

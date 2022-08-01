// runs in debug mode, so the _read() needs to be called manually

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugSwitchUser : CommonSpec
    {
        [Test]
        public void TestDebugSwitchUser()
        {
            var r = DebugSwitchUser.FromFile(SourceFile("nav_parent_switch.bin"));
            r._read();


            Assert.AreEqual(r.Code, 1);
            Assert.AreEqual(((DebugSwitchUser.One) (r.Data)).Val, -190);
        }
    }
}

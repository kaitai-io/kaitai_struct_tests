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

            // --debug implies --no-auto-read
            r._read();

            Assert.AreEqual(r.Code, 1);
            Assert.AreEqual(((DebugSwitchUser.One) (r.Data)).Val, -190);
        }
    }
}

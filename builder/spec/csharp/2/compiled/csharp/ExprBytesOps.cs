// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

using System.Linq;

namespace Kaitai
{
    public partial class ExprBytesOps : KaitaiStruct
    {
        public static ExprBytesOps FromFile(string fileName)
        {
            return new ExprBytesOps(new KaitaiStream(fileName));
        }

        public ExprBytesOps(KaitaiStream p__io, KaitaiStruct p__parent = null, ExprBytesOps p__root = null) : base(p__io)
        {
            m_parent = p__parent;
            m_root = p__root ?? this;
            f_twoLast = false;
            f_twoMax = false;
            f_oneMin = false;
            f_oneFirst = false;
            f_oneMid = false;
            f_two = false;
            f_twoMin = false;
            f_twoMid = false;
            f_oneSize = false;
            f_oneLast = false;
            f_twoSize = false;
            f_oneMax = false;
            f_twoFirst = false;
            _read();
        }
        private void _read()
        {
            _one = m_io.ReadBytes(3);
        }
        private bool f_twoLast;
        private byte _twoLast;
        public byte TwoLast
        {
            get
            {
                if (f_twoLast)
                    return _twoLast;
                _twoLast = (byte) (Two[Two.Count - 1]);
                f_twoLast = true;
                return _twoLast;
            }
        }
        private bool f_twoMax;
        private byte _twoMax;
        public byte TwoMax
        {
            get
            {
                if (f_twoMax)
                    return _twoMax;
                _twoMax = (byte) (Two.Max());
                f_twoMax = true;
                return _twoMax;
            }
        }
        private bool f_oneMin;
        private byte _oneMin;
        public byte OneMin
        {
            get
            {
                if (f_oneMin)
                    return _oneMin;
                _oneMin = (byte) (One.Min());
                f_oneMin = true;
                return _oneMin;
            }
        }
        private bool f_oneFirst;
        private byte _oneFirst;
        public byte OneFirst
        {
            get
            {
                if (f_oneFirst)
                    return _oneFirst;
                _oneFirst = (byte) (One[0]);
                f_oneFirst = true;
                return _oneFirst;
            }
        }
        private bool f_oneMid;
        private byte _oneMid;
        public byte OneMid
        {
            get
            {
                if (f_oneMid)
                    return _oneMid;
                _oneMid = (byte) (One[1]);
                f_oneMid = true;
                return _oneMid;
            }
        }
        private bool f_two;
        private byte[] _two;
        public byte[] Two
        {
            get
            {
                if (f_two)
                    return _two;
                _two = (byte[]) (new byte[] { 65, 67, 75 });
                f_two = true;
                return _two;
            }
        }
        private bool f_twoMin;
        private byte _twoMin;
        public byte TwoMin
        {
            get
            {
                if (f_twoMin)
                    return _twoMin;
                _twoMin = (byte) (Two.Min());
                f_twoMin = true;
                return _twoMin;
            }
        }
        private bool f_twoMid;
        private byte _twoMid;
        public byte TwoMid
        {
            get
            {
                if (f_twoMid)
                    return _twoMid;
                _twoMid = (byte) (Two[1]);
                f_twoMid = true;
                return _twoMid;
            }
        }
        private bool f_oneSize;
        private int _oneSize;
        public int OneSize
        {
            get
            {
                if (f_oneSize)
                    return _oneSize;
                _oneSize = (int) (One.Count);
                f_oneSize = true;
                return _oneSize;
            }
        }
        private bool f_oneLast;
        private byte _oneLast;
        public byte OneLast
        {
            get
            {
                if (f_oneLast)
                    return _oneLast;
                _oneLast = (byte) (One[One.Count - 1]);
                f_oneLast = true;
                return _oneLast;
            }
        }
        private bool f_twoSize;
        private int _twoSize;
        public int TwoSize
        {
            get
            {
                if (f_twoSize)
                    return _twoSize;
                _twoSize = (int) (Two.Count);
                f_twoSize = true;
                return _twoSize;
            }
        }
        private bool f_oneMax;
        private byte _oneMax;
        public byte OneMax
        {
            get
            {
                if (f_oneMax)
                    return _oneMax;
                _oneMax = (byte) (One.Max());
                f_oneMax = true;
                return _oneMax;
            }
        }
        private bool f_twoFirst;
        private byte _twoFirst;
        public byte TwoFirst
        {
            get
            {
                if (f_twoFirst)
                    return _twoFirst;
                _twoFirst = (byte) (Two[0]);
                f_twoFirst = true;
                return _twoFirst;
            }
        }
        private byte[] _one;
        private ExprBytesOps m_root;
        private KaitaiStruct m_parent;
        public byte[] One { get { return _one; } }
        public ExprBytesOps M_Root { get { return m_root; } }
        public KaitaiStruct M_Parent { get { return m_parent; } }
    }
}

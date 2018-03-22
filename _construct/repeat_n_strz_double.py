Struct(
	'qty' / Int32ul,
	'lines1' / Array(this.qty // 2, CString(encoding='utf-8')),
	'lines2' / Array(this.qty // 2, CString(encoding='utf-8')),
)

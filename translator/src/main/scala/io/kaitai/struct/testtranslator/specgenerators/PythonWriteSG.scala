package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError, EndOfStreamError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.PythonCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.PythonTranslator

class PythonWriteSG(spec: TestSpec, provider: ClassTypeProvider) extends SpecGenerator {
  importList.add("import unittest")
  importList.add("from specwrite.common_spec import CommonSpec")

  val config = RuntimeConfig()
  val translator = new PythonTranslator(provider, importList, config)
  val className = PythonCompiler.type2class(spec.id)

  override def fileName(name: String): String = s"specwrite/test_$name.py"

  override def indentStr: String = "    "

  override def run(): Unit = {
    out.puts
    out.puts(s"from testwrite.${spec.id} import $className")
    out.puts
    val testClassName = s"Test$className"
    out.puts(s"class $testClassName(CommonSpec.Base):")
    out.inc

    out.puts("def __init__(self, *args, **kwargs):")
    out.inc
    out.puts(s"super($testClassName, self).__init__(*args, **kwargs)")
    out.puts(s"self.struct_class = $className")
    out.puts(s"self.src_filename = 'src/${spec.data}'")
    out.dec

    spec.exception match {
      case None =>
      case Some(_) =>
        out.puts
        out.puts("def test_read_write_roundtrip(self):")
        out.inc
        out.puts("""self.skipTest("cannot use roundtrip because parsing is expected to fail")""")
        out.dec
    }
  }

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}

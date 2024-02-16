package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import _root_.io.kaitai.struct.exprlang.Ast.expr
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.translators.{NimTranslator, TypeDetector}
import _root_.io.kaitai.struct.Utils
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.languages.NimCompiler.{ksToNim, namespaced}

class NimSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = Utils.upperCamelCase(spec.id)
  val translator = new NimTranslator(provider, importList)

  spec.extraImports.foreach(entry =>
    importList.add(s"../../compiled/nim/$entry")
  )

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.BaseGenerator
  override def fileName(name: String): String = s"t${spec.id}.nim"
  override def header(): Unit = {
    out.puts(s"let r = ${className}.fromFile" + "(\"../../src/" + spec.data + "\")")
  }
  override def footer(): Unit = { }
  override def nullAssert(actual: expr): Unit = {
    val actStr = translateAct(actual)
    val td = new TypeDetector(provider)
    val expStr = td.detectType(actual) match {
      case Int1Type(false) => "0'u8"
      case IntMultiType(false, Width2, _) => "0'u16"
      case IntMultiType(false, Width4, _) => "0'u32"
      case IntMultiType(false, Width8, _) => "0'u64"

      case Int1Type(true) => "0'i8"

      case IntMultiType(true, Width2, _) => "0'i16"
      case IntMultiType(true, Width4, _) => "0'i32"
      case IntMultiType(true, Width8, _) => "0'i64"

      case FloatMultiType(Width4, _) => "0'f32"
      case FloatMultiType(Width8, _) => "0'f64"

      case BitsType(_, _) => "0'u64"

      case _: BooleanType => "false"
      case CalcIntType => "0"
      case CalcFloatType => "0'f64"

      case _: StrType => "\"\""
      case _: BytesType => "\"\""

      case KaitaiStructType | _: CalcKaitaiStructType => "nil"
      case KaitaiStreamType => "nil"

      case t: UserType => "nil"
    }

    out.puts(s"assert $actStr == $expStr")
  }
  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assert $actStr == $expStr")
  }
  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[expr]): Unit = {
    val arr = elts.map(v => translator.translate(v))
    val first = s"${ksToNim(elType)}(${arr.head})"
    val rvalue = {
      if (arr.size == 0)
        s"@[]"
      else
        s"@[${first + ", " + arr.tail.mkString(", ")}]"
    }
    out.puts(s"assert ${translateAct(check.actual)} == $rvalue")
  }
  override def runParse(): Unit = {
    importList.add("os, streams, options, sequtils")
    importList.add("../../compiled/nim/" + spec.id)
    importList.add("auxiliary/test_utils")
  }

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.SpecGenerator
  override def indentStr: String = "  "

  override def results: String = {
    "# " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"import $x").mkString("", "\n", "\n") + "\n" + out.result
  }

  // Members declared here
  def translateAct(x: expr) =
    translator.translate(x).replace(Utils.lowerCamelCase(Main.INIT_OBJ_NAME), "r").replace("this.", "");
}

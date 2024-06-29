package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, JSON, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.RustCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.RustTranslator
import _root_.io.kaitai.struct.datatype.DataType.{ArrayType, BooleanType, BytesType, EnumType, IntType, SwitchType, UserType}
import _root_.io.kaitai.struct.format.ClassSpecs
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}

class RustSG(spec: TestSpec, provider: ClassTypeProvider, classSpecs: ClassSpecs) extends BaseGenerator(spec) {
  val className: String = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())
  val compiler = new RustCompiler(provider, RuntimeConfig())
  var do_not_deref = false

  override def fileName(name: String): String = s"tests/test_$name.rs"

  override def header(): Unit = {
    importList.add("#![allow(unused_variables)]")
    importList.add("#![allow(unused_assignments)]")
    importList.add("#![allow(overflowing_literals)]")
    importList.add("use std::fs;")
    importList.add("extern crate kaitai;")
    importList.add("use self::kaitai::*;")
    importList.add(s"use rust::formats::${spec.id}::*;")

    spec.extraImports.foreach{ name => importList.add(s"use rust::formats::$name::*;") }

    out.puts("#[test]")
    out.puts(s"fn test_${spec.id}() -> KResult<()> {")
    out.inc
    out.puts(s"""let bytes = fs::read("../../src/${spec.data}").unwrap();""")
    out.puts("let _io = BytesReader::from(bytes);")
  }

  override def runParse(): Unit = {
    out.puts(s"let r: OptRc<$className> = $className::read_into(&_io, None, None)?;")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    out.puts(s"let res: KResult<OptRc<$className>> = $className::read_into(&_io, None, None);")
    out.puts
    out.puts(s"if let Err(err) = res {")
    out.inc
    out.puts(s"""println!("expected err: {:?}, exception: $exception", err);""")
    out.dec
    out.puts("} else {")
    out.inc
    out.puts(s"""panic!("no expected exception: $exception");""")
    out.dec
    out.puts("}")
  }

  override def footer(): Unit = {
    out.puts("Ok(())")
    out.dec
    out.puts("}")
  }

  def correctIO(code: String): String = {
    var s = if (!do_not_deref) {
      if (code.contains("_io,") && (code.charAt(0) != '*'))  s"*$code" else code
    } else {
      code
    }

    s = s.replace("_io", "&_io")
    s
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actType = translator.detectType(check.actual)
    var actStr = translateAct(check.actual)
    val expType = translator.detectType(check.expected)
    var expStr = translator.translate(check.expected)
    (actType, expType) match {
      case (at: EnumType, et: EnumType) =>
        expStr = remove_ref(expStr)
      case (at: EnumType, et: BooleanType) =>
        expStr = remove_ref(expStr)
      case (at: EnumType, et: IntType) =>
        out.puts(s"let n : i64 = (&$actStr).into();")
        actStr = s"n"
      case _ =>
    }
    // fix expStr as vector
    if (actStr.charAt(0) == '*' && expStr.startsWith("&vec![")) {
      expStr = remove_ref(expStr)
    }
    //TODO: correct code generation
    actStr = correctIO(actStr)
    out.puts(s"assert_eq!($actStr, $expStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translator.remove_deref(correctIO(translateAct(actual)))
    out.puts(s"assert!($actStr.is_none());")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleEquality(check)
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    val s = translator.remove_deref(correctIO(translateAct(actual).replace(")?", ").unwrap_err()")))
    out.puts(s"assert_eq!($s, ${compiler.ksErrorName(exception)});")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translate(x: Ast.expr): String = {
    val ttx = translator.translate(x)
    val dots = ttx.split("\\.")
    var ttx2 = dots(0)
    var last = ""
    var last_full = ""
    dots.drop(1).foreach {
      attr_full =>
        last_full = attr_full
        val ind = attr_full indexOf "("
        if (ind > 0) {
          val attr = attr_full.substring(0, ind)
          last = attr
        }
        ttx2 = s"$ttx2.$attr_full"
    }
    // do we need to deref?
    if (last.nonEmpty) {
      var deref = true
      if (last == "len" || last_full.contains("[")) {
        deref = false
        do_not_deref = true
      } else {
        deref = translator.need_deref(last, classSpecs.firstSpec)
      }
      if (deref) {
        if (ttx2.charAt(0) == '*') {
          ttx2
        } else {
          s"*$ttx2"
        }
      } else {
        s"${translator.remove_deref(ttx2)}"
      }
    } else {
      ttx
    }
  }

  def remove_ref(s: String): String = {
    if (s.charAt(0) == '&') {
      s.substring(1)
    } else {
      s
    }
  }

  def translateAct(x: Ast.expr): String =
    translate(x).replace(s"self.${Main.INIT_OBJ_NAME}()", "r")
}

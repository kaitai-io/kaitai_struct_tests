package io.kaitai.struct.testtranslator

import java.io.FileReader
import io.kaitai.struct.datatype.KSError
import io.kaitai.struct.exprlang.{Ast, Expressions}
import io.kaitai.struct.format.ParseUtils
import io.kaitai.struct.formats.JavaKSYParser
import io.kaitai.struct.problems.{KSYParseError, ProblemCoords, YAMLParserError}

sealed trait TestAssert
case class TestEquals(actual: Ast.expr, expected: Ast.expr) extends TestAssert
case class TestException(actual: Ast.expr, exception: KSError) extends TestAssert
/** Checks that `to-string` expression produces the specified string. */
case class TestToString(expected: String) extends TestAssert

case class TestSpec(
  id: String,
  data: String,
  asserts: List[TestAssert],
  exception: Option[KSError],
  extraImports: List[String]
)

object TestSpec {
  def testAssertFromYaml(src: Any, path: List[String]): TestAssert = {
    val srcMap = ParseUtils.asMapStr(src, path)

    val expStr = ParseUtils.getOptValueStr(srcMap, "expected", path)
    val excStr = ParseUtils.getOptValueStr(srcMap, "exception", path)
    val tosStr = ParseUtils.getOptValueStr(srcMap, "to-string", path)

    (expStr, excStr, tosStr) match {
      case (Some(exp), None, None) =>
        val actStr = ParseUtils.getValueStr(srcMap, "actual", path)
        val actExp = Expressions.parse(Main.INIT_OBJ_NAME + "." + actStr)
        val expExp = Expressions.parse(exp)
        TestEquals(actExp, expExp)
      case (None, Some(exc), None) =>
        val actStr = ParseUtils.getValueStr(srcMap, "actual", path)
        val actExp = Expressions.parse(Main.INIT_OBJ_NAME + "." + actStr)
        val exception = KSError.fromName(exc)
        TestException(actExp, exception)
      case (None, None, Some(tos)) =>
        TestToString(tos)

      case (Some(_), Some(_), Some(_)) =>
        throw KSYParseError("only one of `expected`, `exception` or `to-string` are allowed", path).toException
      case (None, Some(_), Some(_)) =>
        throw KSYParseError("can't have both `exception` and `to-string`", path).toException
      case (Some(_), None, Some(_)) =>
        throw KSYParseError("can't have both `expected` and `to-string`", path).toException
      case (Some(_), Some(_), None) =>
        throw KSYParseError("can't have both `expected` and `exception`", path).toException

      case (None, None, None) =>
        throw KSYParseError("need either `expected`, `exception`, or `to-string`", path).toException
    }
  }

  def fromYaml(src: Any): TestSpec = {
    val srcMap = ParseUtils.asMapStr(src, List())

    val id = ParseUtils.getValueStr(srcMap, "id", List())
    val data = ParseUtils.getValueStr(srcMap, "data", List())
    val asserts = ParseUtils.getList[TestAssert](srcMap, "asserts", testAssertFromYaml, List())
    val exception = ParseUtils.getOptValueStr(srcMap, "exception", List()).map(KSError.fromName)
    val extraImports = ParseUtils.getListStr(srcMap, "imports", List())

    TestSpec(id, data, asserts, exception, extraImports)
  }

  def fromFile(fileName: String): TestSpec = {
    val reader = new FileReader(fileName)
    val src = JavaKSYParser.readerToYaml(reader)
    fromYaml(src)
  }
}

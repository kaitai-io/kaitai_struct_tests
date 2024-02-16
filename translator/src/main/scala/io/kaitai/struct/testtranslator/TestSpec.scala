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

    val actStr = ParseUtils.getValueStr(srcMap, "actual", path)
    val actExp = Expressions.parse(Main.INIT_OBJ_NAME + "." + actStr)

    val expStr = ParseUtils.getOptValueStr(srcMap, "expected", path)
    val excStr = ParseUtils.getOptValueStr(srcMap, "exception", path)

    (expStr, excStr) match {
      case (Some(exp), None) =>
        val expExp = Expressions.parse(exp)
        TestEquals(actExp, expExp)
      case (None, Some(exc)) =>
        val exception = KSError.fromName(exc)
        TestException(actExp, exception)
      case (Some(_), Some(_)) =>
        throw KSYParseError("can't have both `expected` and `exception`", path).toException
      case (None, None) =>
        throw KSYParseError("need either `expected` or `exception`", path).toException
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

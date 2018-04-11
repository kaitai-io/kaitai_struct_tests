package io.kaitai.struct.testtranslator

import io.kaitai.struct.exprlang.{Ast, Expressions}
import io.kaitai.struct.format.ParseUtils

case class TestAssert(actual: Ast.expr, expected: Ast.expr)

case class TestSpec(id: String, data: String, asserts: List[TestAssert])

object TestSpec {
  def testAssertFromYaml(src: Any, path: List[String]): TestAssert = {
    val srcMap = ParseUtils.asMapStr(src, path)

    val actStr = ParseUtils.getValueStr(srcMap, "actual", path)
    val expStr = ParseUtils.getValueStr(srcMap, "expected", path)

    val actExp = Expressions.parse(Main.INIT_OBJ_NAME + "." +actStr)
    val expExp = Expressions.parse(expStr)

    TestAssert(actExp, expExp)
  }

  def fromYaml(src: Any): TestSpec = {
    val srcMap = ParseUtils.asMapStr(src, List())

    val id = ParseUtils.getValueStr(srcMap, "id", List())
    val data = ParseUtils.getValueStr(srcMap, "data", List())
    val asserts = ParseUtils.getList[TestAssert](srcMap, "asserts", testAssertFromYaml, List())

    TestSpec(id, data, asserts)
  }
}

package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.datatype.{DataType, KSError, EndOfStreamError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.ZigCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestException, TestSpec, ExpectedException}
import io.kaitai.struct.translators.ZigTranslator

class ZigSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val config = RuntimeConfig()
  val className = ZigCompiler.type2class(spec.id)
  val translator = new ZigTranslator(provider, importList, config)
  val compiler = new ZigCompiler(provider, config)

  importList.add("""const _imp_std = @import("std");""")
  importList.add("""const _imp_kaitai_struct = @import("kaitai_struct");""")
  // NOTE: this is not the correct final path, that would be
  // `@import("../formats/${spec.id}.zig")`. However, since we also pass
  // `importList` to ZigTranslator, which will generate
  // `@import("${spec.id}.zig")` anyway, we might as well embrace that
  // convention and fix the paths when printing the `importList` to the
  // resulting test spec file.
  importList.add(s"""const _imp_${spec.id} = @import("${spec.id}.zig");""")
  // NOTE: some other languages use `spec.extraImports` here, but since we pass
  // `importList` to ZigTranslator above, which will insert necessary imports
  // when translating external enum literals itself, we don't need to use it
  // here. It would only generate redundant imports.

  override def fileName(name: String): String = s"tests/${name}_test.zig"

  override def header(): Unit = {
    out.puts(s"""test "$className" {""")
    out.inc
  }

  override def runParse(): Unit = {
    runParseCommon()
    out.puts(s"const r = try _imp_${spec.id}.$className.create(&arena, &_io, null, null);")
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    runParseCommon()
    out.puts(s"try _imp_std.testing.expectError(${compiler.ksErrorName(exception)}, _imp_${spec.id}.$className.create(&arena, &_io, null, null));")
  }

  def runParseCommon(): Unit = {
    out.puts(s"""const file = try _imp_std.fs.cwd().openFile("../../src/${spec.data}", .{});""")
    out.puts("defer file.close();")
    out.puts("var buffer: [8]u8 = undefined;")
    out.puts("var reader = file.reader(&buffer);")
    out.puts("const allocator = _imp_std.testing.allocator;")
    out.puts("var arena = _imp_std.heap.ArenaAllocator.init(allocator);")
    out.puts("defer arena.deinit();")
    out.puts("var _io = _imp_kaitai_struct.KaitaiStream.fromFileReader(&reader);")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actType = translator.detectType(check.actual)
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    actType match {
      case _: BytesType =>
        out.puts(s"try _imp_std.testing.expectEqualSlices(u8, $expStr, $actStr);")
      case _: StrType =>
        out.puts(s"try _imp_std.testing.expectEqualStrings($expStr, $actStr);")
      case _ =>
        out.puts(s"try _imp_std.testing.expectEqual($expStr, $actStr);")
    }
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    val actStrWithoutUnwrap = actStr.stripSuffix(".?")
    out.puts(s"try _imp_std.testing.expectEqual(null, $actStrWithoutUnwrap);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    val elTypeName = ZigCompiler.kaitaiType2NativeType(elType, importList, provider.nowClass)
    val actStr = translateAct(check.actual)
    val eltsStr = elts.map((x) => translator.translate(x)).mkString(", ")
    out.puts(s"try _imp_std.testing.expectEqualSlices($elTypeName, &.{ $eltsStr }, $actStr.items);")
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    out.puts(s"assertThrows(${compiler.ksErrorName(exception)}.class, new ThrowingRunnable() {")
    out.inc

    out.puts("@Override")
    out.puts("public void run() throws Throwable {")
    out.inc
    out.puts(s"${translateAct(actual)};")
    out.dec
    out.puts("}")

    out.dec
    out.puts("});")
  }

  override def noAsserts() =
    out.puts("_ = r;")

  override def indentStr: String = "    "

  override def results: String = {
    import scala.util.matching.Regex

    val formatImportPattern: Regex = """^const _imp_(.*?) = @import\("\1\.zig"\);$""".r

    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map { x =>
        x match {
          case formatImportPattern(format) =>
            s"""const _imp_$format = @import("../formats/$format.zig");"""
          case _ => x
        }
      }.mkString("\n") + "\n\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x)
      .replace("self." + Main.INIT_OBJ_NAME + ".", "r.")
      .replaceAll("\\bself\\._allocator\\(\\)", "allocator")
}

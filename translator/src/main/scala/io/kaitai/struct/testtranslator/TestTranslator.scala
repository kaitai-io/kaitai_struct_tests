package io.kaitai.struct.testtranslator

import java.io.{File, FileWriter}
import io.kaitai.struct.JavaMain.CLIConfig
import io.kaitai.struct.datatype.DataType.UserTypeInstream
import io.kaitai.struct.format._
import io.kaitai.struct.formats.JavaKSYParser
import io.kaitai.struct.problems.ProblemSeverity
import io.kaitai.struct.testtranslator.Main.CLIOptions
import io.kaitai.struct.testtranslator.specgenerators._
import io.kaitai.struct.{ClassTypeProvider, CppRuntimeConfig}

import scala.collection.immutable.SortedMap

class TestTranslator(options: CLIOptions) {
  import Main._

  def run(): Unit = {
    options.srcFiles.foreach(testName =>
      doTestSpec(testName, options.targets, options.outDir)
    )
  }

  def doTestSpec(testName: String, langs: Seq[String], outDir: String): Unit = {
    Console.println(s"Translating: $testName")

    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    langs.foreach(langName => {
      val sg = getSG(langName, testSpec, provider)
      val langDir = langName.stripSuffix("-write")
      try {
        val outFile = s"$outDir/$langDir/${sg.fileName(testName)}"
        Console.println(s"... generating $outFile")
        sg.run()
        writeFile(outFile, sg.results)
      } catch {
        case e: Throwable => e.printStackTrace(Console.err)
      }
    })
  }

  def writeFile(fileName: String, contents: String): Unit = {
    val outPath = new File(fileName)

    // Ensure that all directories leading to this path exist
    val parentPath = outPath.getParentFile
    parentPath.mkdirs

    val fw = new FileWriter(outPath)
    fw.write(contents)
    fw.close()
  }

  def loadTestSpec(testName: String): TestSpec =
    TestSpec.fromFile(s"$specKsDir/$testName.kst")

  def loadClassSpecs(testName: String): ClassSpecs = {
    val cliConfig = CLIConfig(importPaths = importDirs)
    val (origSpecsOpt, errors) = JavaKSYParser.localFileToSpecs(s"$formatsDir/$testName.ksy", cliConfig)

    errors.foreach(problem => Console.err.println(problem.message))
    if (errors.exists(problem => problem.severity != ProblemSeverity.Warning) || origSpecsOpt.isEmpty) {
      throw new RuntimeException("Error during localFileToSpecs")
    }

    val origSpecs = origSpecsOpt.get

    val userType = UserTypeInstream(origSpecs.firstSpec.name, None)
    userType.classSpec = Some(origSpecs.firstSpec)

    val initObj = ClassSpec(
      fileName = None,
      path = List(),
      isTopLevel = true,
      meta = MetaSpec(
        path = List("meta"),
        isOpaque = false,
        id = Some(INIT_OBJ_TYPE),
        endian = None,
        bitEndian = None,
        encoding = None,
        forceDebug = false,
        opaqueTypes = None,
        zeroCopySubstream = None,
        imports = List()
      ),
      DocSpec.EMPTY,
      toStringExpr = None,
      params = List(),
      seq = List(
        AttrSpec(
          path = List("seq", "0"),
          id = NamedIdentifier(INIT_OBJ_NAME),
          dataType = userType
        )
      ),
      types = SortedMap(),
      instances = SortedMap(),
      enums = SortedMap()
    )

    initObj.name = List(INIT_OBJ_TYPE)

    origSpecs.put(INIT_OBJ_TYPE, initObj)

    origSpecs
  }

  def getSG(lang: String, testSpec: TestSpec, provider: ClassTypeProvider): SpecGenerator = lang match {
    case "construct" => new ConstructSG(testSpec, provider)
    case "cpp_stl_98" => new CppStlSG(testSpec, provider, CppRuntimeConfig().copyAsCpp98())
    case "cpp_stl_11" => new CppStlSG(testSpec, provider, CppRuntimeConfig().copyAsCpp11())
    case "csharp" => new CSharpSG(testSpec, provider)
    case "go" => new GoSG(testSpec, provider)
    case "java" => new JavaSG(testSpec, provider)
    case "java-write" => new JavaWriteSG(testSpec, provider)
    case "javascript" => new JavaScriptSG(testSpec, provider)
    // case "julia" => new JuliaSG(testSpec, provider)
    case "lua" => new LuaSG(testSpec, provider)
    case "nim" => new NimSG(testSpec, provider)
    case "perl" => new PerlSG(testSpec, provider)
    case "php" => new PHPSG(testSpec, provider)
    case "python" => new PythonSG(testSpec, provider)
    case "python-write" => new PythonWriteSG(testSpec, provider)
    case "ruby" => new RubySG(testSpec, provider)
    case "rust" => new RustSG(testSpec, provider)
    case "zig" => new ZigSG(testSpec, provider)
  }
}

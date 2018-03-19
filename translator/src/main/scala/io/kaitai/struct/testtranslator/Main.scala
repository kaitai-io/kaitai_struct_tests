package io.kaitai.struct.testtranslator

import java.io.{File, FileReader, FileWriter}

import io.kaitai.struct.ClassTypeProvider
import io.kaitai.struct.JavaMain.CLIConfig
import io.kaitai.struct.datatype.DataType.UserTypeInstream
import io.kaitai.struct.format._
import io.kaitai.struct.formats.JavaKSYParser
import io.kaitai.struct.testtranslator.specgenerators._

object Main extends App {
  val baseDir = ".."
  val specKsDir = s"$baseDir/spec/ks"
  val formatsDir = s"$baseDir/formats"
  val outDir = s"$specKsDir/out"

  def writeFile(fileName: String, contents: String): Unit = {
    val outPath = new File(fileName)

    // Ensure that all directories leading to this path exist
    val parentPath = outPath.getParentFile
    parentPath.mkdirs

    val fw = new FileWriter(outPath)
    fw.write(contents)
    fw.close()
  }

  def loadTestSpec(testName: String): TestSpec = {
    val reader = new FileReader(s"$specKsDir/$testName.kst")
    val src = JavaKSYParser.readerToYaml(reader)
    TestSpec.fromYaml(src)
  }

  final val INIT_OBJ_TYPE = "unique_top_level_container"
  final val INIT_OBJ_NAME = "q1w2e3"

  def loadClassSpecs(testName: String): ClassSpecs = {
    val origSpecs = JavaKSYParser.localFileToSpecs(s"$formatsDir/$testName.ksy", CLIConfig())

    val userType = UserTypeInstream(origSpecs.firstSpec.name, None)
    userType.classSpec = Some(origSpecs.firstSpec)

    val initObj = ClassSpec(
      path = List(),
      isTopLevel = true,
      meta = MetaSpec(
        path = List("meta"),
        isOpaque = false,
        id = Some(INIT_OBJ_TYPE),
        endian = None,
        encoding = None,
        forceDebug = false,
        opaqueTypes = None,
        imports = List()
      ),
      DocSpec.EMPTY,
      params = List(),
      seq = List(
        AttrSpec(
          path = List("seq", "0"),
          id = NamedIdentifier(INIT_OBJ_NAME),
          dataType = userType
        )
      ),
      types = Map(),
      instances = Map(),
      enums = Map()
    )

    initObj.name = List(INIT_OBJ_TYPE)

    origSpecs.put(INIT_OBJ_TYPE, initObj)

    origSpecs
  }

  def doAllSpecs(testName: String): Unit = {
    Console.println(s"doAllSpecs($testName)")

    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    val sgs = Map(
      "cpp_stl" -> new CppStlSG(testSpec, provider),
      //"go" -> new GoSG(testSpec, provider),
      "java" -> new JavaSG(testSpec, provider),
      "javascript" -> new JavaScriptSG(testSpec, provider),
      "php" -> new PHPSG(testSpec, provider),
      "python" -> new PythonSG(testSpec, provider),
      "ruby" -> new RubySG(testSpec, provider)
    )

    sgs.foreach { case (langName, sg) =>
      try {
        sg.run()
        writeFile(s"$outDir/$langName/${sg.fileName(testName)}", sg.results)
      } catch {
        case e => e.printStackTrace(Console.err)
      }
    }
  }

  def doOneSpec(langName: String, testName: String) {
    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

//    val sg = new PythonSG(testSpec, provider)
    val sg = new CppStlSG(testSpec, provider)
    sg.run()

    writeFile(outDir + "/" + langName + "/" + sg.fileName(testName), sg.results)
  }

  def doAll(): Unit = {
    val dir = new File(specKsDir)
    dir.list().foreach((fn) =>
      if (fn.endsWith(".kst"))
        doAllSpecs(fn.substring(0, fn.size - 4))
    )
  }

  //doOneSpec("hello_world")
  //doOneSpec("process_xor4_const")
  //doOneSpec("enum_0")
  //doOneSpec("cpp_stl", "switch_manual_int_else")
  //doAllSpecs("if_values")

  doAll()
}

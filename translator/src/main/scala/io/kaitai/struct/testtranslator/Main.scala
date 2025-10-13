package io.kaitai.struct.testtranslator

import java.io.File

import _root_.io.kaitai.struct.format._
import io.kaitai.struct.Version

object Main extends App {
  val baseDir = ".."
  val specDir = s"$baseDir/spec"
  val specKsDir = s"$baseDir/spec/ks"
  val formatsDir = s"$baseDir/formats"
  val defaultOutDir = s"$specKsDir/out"
  val importDirs = List(s"$baseDir/../formats", s"$formatsDir/ks_path")

  val ALL_LANGS = List(
    "construct",
    "cpp_stl_98",
    "cpp_stl_11",
    "csharp",
    "go",
    "java",
    "java-write",
    "javascript",
    // "julia",
    "lua",
    "nim",
    "perl",
    "php",
    "python",
    "python-write",
    "ruby",
    "rust",
    "zig"
  )

  val INIT_OBJ_TYPE = "unique_top_level_container"
  val INIT_OBJ_NAME = "q1w2e3"

  case class CLIOptions(
    srcFiles: Seq[String] = Seq(),
    targets: Seq[String] = Seq(),
    outDir: String = defaultOutDir
  )

  // Stupid ksc build-time issue: no simple way to get that generated constant
  // into xplatform sources, so we copy it in run-time.
  KSVersion.current = Version.version

  val parser = new scopt.OptionParser[CLIOptions]("kst_translator") {
    override def showUsageOnError = Some(true)

    head("KST translator", KSVersion.current.toString)

    arg[String]("<test_name>...").unbounded().optional().action { (x, c) =>
      val testName: String = if (x.endsWith("kst")) {
        x.substring(0, x.length - 4)
      } else {
        x
      }
      c.copy(srcFiles = c.srcFiles :+ testName)
    } text("source test names (.kst)")

    opt[String]('t', "target").unbounded().valueName("<language>").action { (x, c) =>
      if (x == "all") {
        c.copy(targets = ALL_LANGS)
      } else {
        c.copy(targets = c.targets :+ x)
      }
    } text(s"target languages (${ALL_LANGS.mkString(", ")}, default: all)") validate { x =>
      if (ALL_LANGS.contains(x) || x == "all") {
        success
      } else {
        failure(s"'$x' is not a valid target language; valid ones are: ${ALL_LANGS.mkString(", ")}")
      }
    }

    opt[Unit]("all-specs") action { (_, c) =>
      val dir = new File(specKsDir)
      val list: Array[String] = dir.list().
        filter(_.endsWith(".kst")).
        map(fn => fn.substring(0, fn.length - 4))
      c.copy(srcFiles = list.toSeq)
    } text("process all KST files available")

    opt[Unit]('f', "force") action { (x, c) =>
      c.copy(outDir = specDir)
    } text(s"force overwrite specs in production spec dirs (default: generate in $defaultOutDir)")

    checkConfig(
      c =>
        if (c.srcFiles.isEmpty) failure("no test names found")
        else success)
  }

  parser.parse(args, CLIOptions()) match {
    case None => System.exit(1)
    case Some(config0) =>
      val config = if (config0.targets.isEmpty) {
        config0.copy(targets = ALL_LANGS)
      } else {
        config0
      }
      new TestTranslator(config).run()
  }
}

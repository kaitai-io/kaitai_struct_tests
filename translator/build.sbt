name := "translator"

version := "0.9-SNAPSHOT"

scalaVersion := "2.12.4"

resolvers += "Sonatype OSS Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots"

libraryDependencies ++= Seq(
  "io.kaitai" % "kaitai-struct-compiler_2.12" % "0.9-SNAPSHOT",
  "org.yaml" % "snakeyaml" % "1.16"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")
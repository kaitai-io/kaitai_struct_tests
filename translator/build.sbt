name := "translator"

version := "0.11-SNAPSHOT"

scalaVersion := "2.12.8"

resolvers += "Sonatype OSS Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots"
resolvers ++= Resolver.sonatypeOssRepos("public")

libraryDependencies ++= Seq(
  "io.kaitai" %% "kaitai-struct-compiler" % "0.11-SNAPSHOT",
  "com.github.scopt" %% "scopt" % "4.1.0"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")

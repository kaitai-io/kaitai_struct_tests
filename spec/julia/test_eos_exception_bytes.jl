# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import EosExceptionBytesModule
using KaitaiStruct

@testset "EosExceptionBytes test" begin
    @test_throws ErrorException EosExceptionBytesModule.from_file("src/term_strz.bin")
end
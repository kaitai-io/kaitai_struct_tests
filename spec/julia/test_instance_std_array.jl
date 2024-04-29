# Autogenerated from KST: please remove this line if doing any edits by hand!

using Test
import InstanceStdArrayModule

@testset "InstanceStdArray test" begin
    r = InstanceStdArrayModule.from_file("src/instance_std_array.bin")


    @test r.ofs == 16
    @test r.qty_entries == 3
    @test r.entry_size == 4
    @test Base.size(r.entries, 1) == 3
    @test r.entries[1] == Vector{UInt8}([17, 17, 17, 17])
    @test r.entries[2] == Vector{UInt8}([34, 34, 34, 34])
    @test r.entries[3] == Vector{UInt8}([51, 51, 51, 51])
end
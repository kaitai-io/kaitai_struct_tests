using Test
using TestReports


function run_tests()
    #nested testset is needed to continue even after some test errors
    @testset "" begin
        for f in filter(s -> contains(s, r".jl$") && !contains(s, "runtests.jl"), readdir(@__DIR__; join=true))
            include(f)
        end
    end
end

ts = @testset ReportingTestSet "" begin
    run_tests()
end

if !isempty(ARGS)
    open(ARGS[1], "w") do fh
        print(fh, report(ts))
    end
end
exit(any_problems(ts))

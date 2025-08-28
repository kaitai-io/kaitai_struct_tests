module MyCustomFx
export decode

function decode(key::Integer, flag::Bool, some_bytes::Vector{UInt8}, data::Vector{UInt8})
    _key = flag ? key : -key
    dst = Vector{UInt8}(undef, length(data))
    for i in eachindex(dst)
        dst[i] = (data[i] + _key)
    end
    dst
end
end

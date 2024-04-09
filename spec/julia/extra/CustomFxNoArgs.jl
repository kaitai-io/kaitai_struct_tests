module CustomFxNoArgs
export decode

function decode(data::Vector{UInt8})
    Vector{UInt8}([UInt8('_'), data..., UInt8('_')])
end
end

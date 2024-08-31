module Nested
module Deeply
module CustomFx

export decode

function decode(foo, data::Vector{UInt8})
    Vector{UInt8}([UInt8('_'), data..., UInt8('_')])
end

end
end
end

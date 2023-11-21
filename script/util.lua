Util = {}

function Util.split(str, sep)
    local sep, res = sep or '%s', {}
    string.gsub(str, '[^' .. sep .. ']+', function(x) res[#res + 1] = x end)
    return res
end

function Util.indexOf(array, value)
    for i, v in ipairs(array) do
        if v == value then
            return i
        end
    end
    return nil
end

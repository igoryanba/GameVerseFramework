fixture_load_order = fixture_load_order or {}
table.insert(fixture_load_order, "shared")

RegisterCallback("fixture:add", function(a, b)
    return a + b
end)

exports("fixture_answer", function()
    return 42
end)

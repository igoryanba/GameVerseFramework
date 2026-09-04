table.insert(fixture_load_order, "client")

RegisterNetEvent("fixture:server-ready", function(value)
    TriggerServerEvent("fixture:client-ack", value + 1)
end)

CreateThread(function()
    Wait(5)
    TriggerServerEvent("fixture:thread", true)
end)

SetTimeout(10, function()
    TriggerServerEvent("fixture:timer", true)
end)

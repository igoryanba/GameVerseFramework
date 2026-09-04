table.insert(fixture_load_order, "server")

RegisterNetEvent("fixture:client-ack", function(value)
    TriggerClientEvent("fixture:accepted", 1, value + 1)
end)

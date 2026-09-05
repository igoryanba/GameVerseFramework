RegisterNetEvent('modern:ready')
RegisterCommand('modern', function() end, false)
local locale = GetConvar('modern:locale', 'en')
SendNUIMessage({ type = 'ready', locale = locale })

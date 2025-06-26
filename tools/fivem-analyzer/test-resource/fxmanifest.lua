fx_version "cerulean"
game "gta5"
name "test-qb-banking"
author "Test Author"
version "1.0.0"

dependencies { "qb-core", "qb-target", "ox_lib" }

shared_scripts { "config.lua" }
client_scripts { "client/*.lua" }
server_scripts { "server/*.lua" }

ui_page "html/index.html"
files { "html/style.css", "html/script.js" }

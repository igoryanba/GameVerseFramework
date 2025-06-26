// GameVerse Basic Gamemode - Server Main
// Базовый геймод с функционалом spawn, chat, admin

/// <reference types="@gameverse/types" />

// Временные определения типов для разработки
interface Vector3 {
    x: number;
    y: number;
    z: number;
}

// Mock GameVerse API для разработки (будет заменено реальным API)
const onNet = (event: string, handler: (...args: any[]) => void) => {
    console.log(`Registered event: ${event}`);
};

const emit = (event: string, target: string, ...args: any[]) => {
    console.log(`Emitting ${event} to ${target}:`, args);
};

const getPlayers = (): string[] => {
    return Array.from(players.keys());
};

// Конфигурация
const config = {
    defaultSpawn: { x: 686.245, y: 577.950, z: 130.461 },
    maxPlayers: 32,
    enableChat: true,
    enableAdmin: true
};

// Состояние сервера
const players = new Map<string, PlayerData>();

interface PlayerData {
    id: string;
    name: string;
    money: number;
    isAdmin: boolean;
    spawnLocation: Vector3;
}

// События игрока
onNet('playerConnecting', (source: string, name: string) => {
    console.log(`🔗 Player ${name} connecting...`);
    
    // Инициализация данных игрока
    players.set(source, {
        id: source,
        name,
        money: 5000, // Стартовые деньги
        isAdmin: false,
        spawnLocation: config.defaultSpawn
    });
});

onNet('playerDropped', (source: string, reason: string) => {
    const player = players.get(source);
    if (player) {
        console.log(`❌ Player ${player.name} disconnected: ${reason}`);
        players.delete(source);
    }
});

// Система spawn
onNet('playerSpawn', (source: string) => {
    const player = players.get(source);
    if (player) {
        console.log(`🎮 Spawning player ${player.name}`);
        
        // Отправляем данные spawn на клиент
        emit('client:setSpawnLocation', source, player.spawnLocation);
        emit('client:setPlayerMoney', source, player.money);
    }
});

// Административные команды
onNet('chatCommand', (source: string, command: string, args: string[]) => {
    const player = players.get(source);
    if (!player) return;

    switch (command) {
        case 'admin':
            if (player.isAdmin) {
                handleAdminCommand(source, args);
            } else {
                emit('client:chatMessage', source, { 
                    color: 'red', 
                    message: 'You are not an admin!' 
                });
            }
            break;

        case 'balance':
            emit('client:chatMessage', source, {
                color: 'green',
                message: `Your balance: $${player.money}`
            });
            break;

        case 'help':
            showHelpMessage(source);
            break;
    }
});

function handleAdminCommand(source: string, args: string[]) {
    const subCommand = args[0];
    
    switch (subCommand) {
        case 'kick':
            // TODO: Implement kick functionality
            break;
        case 'ban':
            // TODO: Implement ban functionality  
            break;
        case 'tp':
            // TODO: Implement teleport functionality
            break;
    }
}

function showHelpMessage(source: string) {
    const helpMessages = [
        '🎮 GameVerse Server Commands:',
        '/balance - Check your money',
        '/help - Show this message',
        '/admin <command> - Admin commands (if admin)',
    ];
    
    helpMessages.forEach(msg => {
        emit('client:chatMessage', source, { color: 'blue', message: msg });
    });
}

// Инициализация сервера
console.log('🚀 GameVerse Basic Gamemode loaded!');
console.log(`📊 Max players: ${config.maxPlayers}`);
console.log(`💬 Chat enabled: ${config.enableChat}`);
console.log(`🛡️ Admin enabled: ${config.enableAdmin}`);

// Экспорт для других ресурсов
export { players, config }; 
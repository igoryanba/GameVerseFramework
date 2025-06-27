import { FiveMCompat } from 'gameverse-sdk';

const compat = new FiveMCompat();

// Подписка на серверное событие chatMessage
compat.on('chatMessage', (source: number, name: string, message: string) => {
  console.log(`[chat] ${name}: ${message}`);
});

// Отправка команды на сервер через событие
compat.triggerServerEvent('clientReady', []);

console.log('Client plugin {{plugin_name}} loaded'); 
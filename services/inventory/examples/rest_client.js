#!/usr/bin/env node

/**
 * Пример использования REST API сервиса инвентаря
 * 
 * Для запуска:
 * node rest_client.js
 */

const fetch = require('node-fetch');

// Базовый URL для API
const API_URL = process.env.INVENTORY_SERVICE_HTTP_URL || 'http://localhost:8081';

// Функция для выполнения HTTP запросов
async function request(method, path, body = null) {
  const options = {
    method,
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
  };

  if (body) {
    options.body = JSON.stringify(body);
  }

  const response = await fetch(`${API_URL}${path}`, options);
  
  if (response.status >= 400) {
    const error = await response.json();
    throw new Error(`API Error: ${error.error || response.statusText}`);
  }
  
  if (response.status === 204) {
    return null; // No Content
  }
  
  return await response.json();
}

// Основная функция примера
async function main() {
  try {
    console.log('==== Пример использования REST API сервиса инвентаря ====');
    
    // 1. Создаем шаблон предмета
    console.log('\n1. Создание шаблона предмета:');
    const itemTemplate = await request('POST', '/templates', {
      name: 'Меч',
      description: 'Острый стальной меч',
      weight: 5.0,
      stackable: false,
      max_stack: 1,
      max_durability: 100.0,
      icon: 'sword.png',
      category: 'weapons',
      properties: {
        damage: '10',
        attack_speed: '1.2'
      }
    });
    
    console.log('Создан шаблон предмета:', itemTemplate);
    
    // 2. Создаем инвентарь
    console.log('\n2. Создание инвентаря:');
    const inventory = await request('POST', '/inventories', {
      owner_id: 'player-123',
      owner_type: 'player',
      inventory_type: 'player',
      max_weight: 100.0,
      max_slots: 20,
      name: 'Рюкзак игрока'
    });
    
    console.log('Создан инвентарь:', inventory);
    
    // 3. Добавляем предмет в инвентарь
    console.log('\n3. Добавление предмета в инвентарь:');
    const item = await request('POST', `/inventories/${inventory.id}/items`, {
      template_id: itemTemplate.id,
      position: 0,
      quantity: 1,
      metadata: {
        equipped: 'false',
        custom_name: 'Меч судьбы'
      }
    });
    
    console.log('Добавлен предмет:', item);
    
    // 4. Получаем все предметы в инвентаре
    console.log('\n4. Получение предметов в инвентаре:');
    const items = await request('GET', `/inventories/${inventory.id}/items`);
    console.log(`Получено ${items.length} предметов:`, items);
    
    // 5. Обновляем предмет
    console.log('\n5. Обновление предмета:');
    const updatedItem = await request('PUT', `/items/${item.id}`, {
      durability: 90.0,
      metadata: {
        equipped: 'true',
        custom_name: 'Меч судьбы +1'
      }
    });
    
    console.log('Обновлен предмет:', updatedItem);
    
    // 6. Создаем еще один шаблон для стакирования
    console.log('\n6. Создание шаблона стакируемого предмета:');
    const stackableTemplate = await request('POST', '/templates', {
      name: 'Зелье лечения',
      description: 'Восстанавливает здоровье',
      weight: 0.5,
      stackable: true,
      max_stack: 10,
      max_durability: 100.0,
      icon: 'potion.png',
      category: 'consumables',
      properties: {
        health_restore: '25'
      }
    });
    
    console.log('Создан стакируемый шаблон:', stackableTemplate);
    
    // 7. Добавляем стакируемый предмет
    console.log('\n7. Добавление стакируемого предмета:');
    const potion1 = await request('POST', `/inventories/${inventory.id}/items`, {
      template_id: stackableTemplate.id,
      position: 1,
      quantity: 5
    });
    
    console.log('Добавлен стакируемый предмет:', potion1);
    
    // 8. Добавляем еще один стакируемый предмет в другой слот
    console.log('\n8. Добавление еще одного стакируемого предмета:');
    const potion2 = await request('POST', `/inventories/${inventory.id}/items`, {
      template_id: stackableTemplate.id,
      position: 2,
      quantity: 3
    });
    
    console.log('Добавлен еще один стакируемый предмет:', potion2);
    
    // 9. Стакируем предметы
    console.log('\n9. Стакирование предметов:');
    await request('POST', `/items/${potion2.id}/stack/${potion1.id}`);
    console.log('Предметы стакированы');
    
    // 10. Проверяем результат стакирования
    console.log('\n10. Проверка результата стакирования:');
    const stackedPotion = await request('GET', `/items/${potion1.id}`);
    console.log('Результат стакирования:', stackedPotion);
    
    // 11. Разделяем стак
    console.log('\n11. Разделение стака:');
    const splitPotion = await request('POST', `/items/${stackedPotion.id}/split`, {
      quantity: 4,
      new_position: 3
    });
    
    console.log('Результат разделения стака:', splitPotion);
    
    // 12. Получаем все предметы после операций
    console.log('\n12. Финальное состояние инвентаря:');
    const finalItems = await request('GET', `/inventories/${inventory.id}/items`);
    console.log(`В инвентаре ${finalItems.length} предметов:`, finalItems);
    
    // 13. Получаем список всех шаблонов предметов
    console.log('\n13. Список всех шаблонов предметов:');
    const templates = await request('GET', '/templates');
    console.log(`Получено ${templates.length} шаблонов:`, templates);
    
    // 14. Получаем шаблоны по категории
    console.log('\n14. Шаблоны по категории "weapons":');
    const weaponTemplates = await request('GET', '/templates?category=weapons');
    console.log(`Получено ${weaponTemplates.length} шаблонов оружия:`, weaponTemplates);
    
    console.log('\n==== Тестирование REST API завершено успешно ====');
  } catch (error) {
    console.error('Ошибка:', error.message);
  }
}

main(); 
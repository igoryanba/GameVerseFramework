#!/bin/bash

# Настройка индексов и маппингов в ElasticSearch для логов GameVerse Framework

# Проверка, запущен ли ElasticSearch
echo "Проверка доступности ElasticSearch..."
until curl -s "http://localhost:9200/_cat/health" > /dev/null; do
    echo "Ожидание запуска ElasticSearch..."
    sleep 5
done

echo "ElasticSearch доступен. Настройка индексов..."

# Создание шаблона индекса для логов
echo "Создание шаблона индекса для логов..."
curl -X PUT "localhost:9200/_template/gameverse_logs" -H 'Content-Type: application/json' -d'
{
  "index_patterns": ["gameverse-logs-*"],
  "settings": {
    "number_of_shards": 1,
    "number_of_replicas": 0,
    "index.lifecycle.name": "gameverse-logs-policy",
    "index.lifecycle.rollover_alias": "gameverse-logs"
  },
  "mappings": {
    "properties": {
      "timestamp": {
        "type": "date"
      },
      "level": {
        "type": "keyword"
      },
      "service": {
        "type": "keyword"
      },
      "component": {
        "type": "keyword"
      },
      "message": {
        "type": "text",
        "fields": {
          "keyword": {
            "type": "keyword",
            "ignore_above": 256
          }
        }
      },
      "context": {
        "properties": {
          "request_id": {
            "type": "keyword"
          },
          "user_id": {
            "type": "keyword"
          },
          "entity_id": {
            "type": "keyword"
          },
          "additional_data": {
            "type": "object",
            "dynamic": true
          }
        }
      },
      "metrics": {
        "properties": {
          "duration_ms": {
            "type": "long"
          },
          "memory_usage": {
            "type": "long"
          }
        }
      }
    }
  }
}'

echo "Создание политики управления жизненным циклом индексов..."
curl -X PUT "localhost:9200/_ilm/policy/gameverse-logs-policy" -H 'Content-Type: application/json' -d'
{
  "policy": {
    "phases": {
      "hot": {
        "min_age": "0ms",
        "actions": {
          "rollover": {
            "max_size": "5GB",
            "max_age": "1d"
          },
          "set_priority": {
            "priority": 100
          }
        }
      },
      "warm": {
        "min_age": "3d",
        "actions": {
          "shrink": {
            "number_of_shards": 1
          },
          "forcemerge": {
            "max_num_segments": 1
          },
          "set_priority": {
            "priority": 50
          }
        }
      },
      "cold": {
        "min_age": "30d",
        "actions": {
          "set_priority": {
            "priority": 0
          }
        }
      },
      "delete": {
        "min_age": "90d",
        "actions": {
          "delete": {}
        }
      }
    }
  }
}'

echo "Создание первоначального индекса..."
curl -X PUT "localhost:9200/gameverse-logs-000001" -H 'Content-Type: application/json' -d'
{
  "aliases": {
    "gameverse-logs": {
      "is_write_index": true
    }
  }
}'

echo "Настройка завершена."
echo "Теперь вы можете отправлять логи в индекс 'gameverse-logs'."
echo "Доступ к Kibana: http://localhost:5601" 
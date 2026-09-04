-- Minimal server-owned content required by the closed-alpha vertical slice.
INSERT INTO item_definitions(id,name,unit_weight_grams,usable) VALUES
    (1,'Вода',500,true),
    (2,'Сэндвич',350,true)
ON CONFLICT(id) DO NOTHING;

INSERT INTO shops(name,enabled) VALUES('market',true)
ON CONFLICT(name) DO UPDATE SET enabled=true;

INSERT INTO shop_items(shop_id,item_id,price)
SELECT shops.id, offers.item_id, offers.price
FROM shops
CROSS JOIN (VALUES (1,120::bigint),(2,200::bigint)) AS offers(item_id,price)
WHERE shops.name='market'
ON CONFLICT(shop_id,item_id) DO UPDATE SET price=EXCLUDED.price;

INSERT INTO jobs(code,enabled) VALUES('courier',true)
ON CONFLICT(code) DO UPDATE SET enabled=true;

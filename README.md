# Проверка коллизий murmur32 для sqlsmith

Тарантул хэширует SQL запросы в откомпилированные выражения с помощью Murmur32 хэша.
Примерная оценка вероятности коллизии для 10к уникальных запросов по 4kkk слотам хэша
составляет 1.24% (см. [парадокс дней рождения](https://ru.wikipedia.org/wiki/%D0%9F%D0%B0%D1%80%D0%B0%D0%B4%D0%BE%D0%BA%D1%81_%D0%B4%D0%BD%D0%B5%D0%B9_%D1%80%D0%BE%D0%B6%D0%B4%D0%B5%D0%BD%D0%B8%D1%8F)),
а для 100k - 71%.
Возникла необходимость проверить эту оценку на практике. Для этого с помощью [sqlsmith](https://github.com/anse1/sqlsmith)
было сгенерировано 101к запросов по стандартной схеме [log.sql](https://github.com/anse1/sqlsmith/blob/master/log.sql)
для PostgreSQL.
```
psql -f log.sql db 
sqlsmith --dry-run --target="host=localhost port=5432 dbname=db" > queries.txt
```
Далее был произведен подсчет количества коллизий с помощью этой тестовой
утилиты:
```sh
collision -f queries.txt > collisions.txt
```
На выходе оказалось две коллизии по хэшам (как и обещала теория).

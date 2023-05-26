# Методы имплементации MongoController и их оптимизация.

Методы `validate_user` и `check_on_something_exist` выполняют одно и то же, хоть и выглядят
по-разному. Внутри себя они создают какой-то фильтр, чтобы далее использовать медот `find_one`.
Потому я хочу их объединить, чтобы они не повторялись, да и лишний код уберу.

```rust
fn validate_user(&self, username: &str, password: &str) -> bool { true } // -
fn check_on_something_exist(&self, filter: Document) -> bool { true } // +
```

---


В целом, стоит просто переопределить методы для всех документов. Начнем с методов типа `GET`:

```rust
// Получить какой-либо документ по чему-либо. В целом, тут просто применяется фильтр.
fn get_document_by<T>(&self, filter: Document) -> Result<T, MongoError>

// Тут параметром функции является фильтр, но он опционален. `filter.unwrap_or(None)`.
fn get_documents<T>(&self, filter: Option<Document>) -> Result<Vec<T>, MongoError>
```

---

На этом методы типа `GET` закончились. Далее у нас методы типа `INSERT`:

```rust
// Добавить какой-либо документ.
fn insert_document<T>(&self, document: T) -> Result<(), MongoError>
fn insert_documents<T>(&self, documents: Vec<T>) -> Result<(), MongoError>
```
Тут понятно. Для одного документа и для множества документов.

---

Так. Далее у меня метод типа `UPDATE`:

```rust
fn update_document<T>(&self, document: T, filter: Document, many: Option<bool>) -> Result<UpdateResponse, MongoError>
```

Метод этого типа принимает 3 значения: `document`, `filter`, `many.unwrap_or(false)`. И возвращает структуру:

```rust
struct UpdateResponse<T> {
    pub reponses: Option<Vec<T>>,
    pub response: Option<T>,
}
```

И последний метод `check_on_document_exist`:

```rust
fn check_on_document_exist<T>(&self, document: Document) -> bool
```

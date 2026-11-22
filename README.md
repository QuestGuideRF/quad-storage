# 📦 quad-storage
**quad-storage** — это библиотека для сохранения данных в постоянном локальном хранилище в окружении miniquad/macroquad. В WASM данные сохраняются даже после закрытия вкладки или браузера благодаря использованию [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API). На нативных платформах данные сохраняются в локальный файл `local.data`.

---

## 📋 Описание проекта

**quad-storage** предоставляет простой и удобный API для работы с локальным хранилищем данных в играх и приложениях, созданных с использованием miniquad или macroquad. Библиотека автоматически определяет платформу и использует соответствующий механизм хранения:

- **WASM (WebAssembly)**: Использует Web Storage API браузера для персистентного хранения
- **Нативные платформы**: Сохраняет данные в локальный файл `local.data`

### ✨ Основные возможности

- 🔒 **Персистентное хранение** — данные сохраняются между сессиями
- 🌐 **Кроссплатформенность** — работает в браузере и на нативных платформах
- 🚀 **Простота использования** — минималистичный и интуитивный API
- ⚡ **Высокая производительность** — эффективная работа с хранилищем
- 🔑 **Полный набор операций** — получение, установка, удаление и очистка данных

---

## 🚀 Установка

Добавьте следующую зависимость в ваш `Cargo.toml`:

```toml
[dependencies]
quad-storage = "0.1.4"
```

---

## 📖 Использование

### Базовый пример

```rust
use quad_storage::*;

// Получить доступ к хранилищу
let storage = &mut STORAGE.lock().unwrap();

// Сохранить данные
storage.set("username", "player123");
storage.set("score", "1000");

// Получить данные
let username = storage.get("username").unwrap();
let score = storage.get("score").unwrap();

println!("Username: {}, Score: {}", username, score);

// Удалить ключ
storage.remove("score");

// Очистить всё хранилище
storage.clear();
```

### Полный пример с обработкой ошибок

```rust
use quad_storage::*;

let storage = &mut STORAGE.lock().unwrap();

// Сохранение данных
storage.set("game_level", "5");
storage.set("player_name", "Alice");

// Получение данных с проверкой
if let Some(level) = storage.get("game_level") {
    println!("Current level: {}", level);
}

// Получение всех ключей
for i in 0..storage.len() {
    if let Some(key) = storage.key(i) {
        if let Some(value) = storage.get(&key) {
            println!("{} = {}", key, value);
        }
    }
}
```

---

## 🌐 Использование в WASM

Для работы в браузере (WASM) необходимо подключить дополнительные JavaScript файлы.

### Шаг 1: Добавьте необходимые файлы

1. Добавьте файл [`js/quad-storage.js`](js/quad-storage.js) в ваш проект
2. Добавьте файл [`sapp-jsutils/js/sapp_jsutils.js`](https://github.com/not-fl3/sapp-jsutils/blob/master/js/sapp_jsutils.js) в ваш проект

### Шаг 2: Подключите скрипты в HTML

Добавьте следующие строки в ваш `index.html` **после** загрузки `gl.js` и **перед** загрузкой вашего WASM файла:

```html
<script src="sapp_jsutils.js"></script>
<script src="quad-storage.js"></script>
```

### Пример полного `index.html`

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>My Game</title>
</head>
<body>
    <script src="gl.js"></script>
    <script src="sapp_jsutils.js"></script>
    <script src="quad-storage.js"></script>
    <script type="module">
        import init from './my_game.js';
        init();
    </script>
</body>
</html>
```

---

## 🔧 API

### Структура `LocalStorage`

Основная структура для работы с хранилищем доступна через глобальный синглтон `STORAGE`.

#### Методы

| Метод | Описание | Возвращаемое значение |
|-------|----------|----------------------|
| `len()` | Получить количество элементов в хранилище | `usize` |
| `key(pos: usize)` | Получить ключ по позиции | `Option<String>` |
| `get(key: &str)` | Получить значение по ключу | `Option<String>` |
| `set(key: &str, value: &str)` | Установить значение для ключа | `()` |
| `remove(key: &str)` | Удалить ключ из хранилища | `()` |
| `clear()` | Очистить всё хранилище | `()` |

### Глобальный доступ

```rust
use quad_storage::STORAGE;

// Получить доступ к хранилищу
let storage = &mut STORAGE.lock().unwrap();
```

---

## 🎯 Особенности

### Платформо-зависимое поведение

- **WASM**: Данные хранятся в `localStorage` браузера и сохраняются между сессиями
- **Нативные платформы**: Данные сохраняются в файл `local.data` в текущей директории

### Потокобезопасность

Хранилище защищено мьютексом (`Mutex`), что обеспечивает безопасную работу в многопоточных приложениях.

### Ограничения

- Все значения хранятся как строки (`String`)
- На нативных платформах файл `local.data` создаётся автоматически при первом сохранении

Подпись относится к текущей версии проекта.

---

Автор: legenda  
Telegram: [@LEGENDA_SD](https://t.me/LEGENDA_SD)

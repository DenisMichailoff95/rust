# Rust Projects Workspace

Общее описание репозитория: в папке находятся три отдельных Rust-проекта:

- `hello_world` — минимальное приложение «Hello, world». 
- `guessing_game` — простая игра от Rust book (угадай число).
- `postgres_console_app` — консольное приложение, работающее с PostgreSQL (может требовать настройки БД/переменных окружения).

Требования

- Установлен Rust (rustup + cargo). Проверка:

```bash
rustc --version
cargo --version
```

Сборка и запуск отдельных проектов

- Сборка `hello_world`:

```bash
cargo build --manifest-path hello_world/Cargo.toml
```

- Запуск `hello_world`:

```bash
cargo run --manifest-path hello_world/Cargo.toml
```

- Сборка `guessing_game`:

```bash
cargo build --manifest-path guessing_game/Cargo.toml
```

- Запуск `guessing_game`:

```bash
cargo run --manifest-path guessing_game/Cargo.toml
```

- Сборка `postgres_console_app`:

```bash
cargo build --manifest-path postgres_console_app/Cargo.toml
```

- Запуск `postgres_console_app`:

```bash
cargo run --manifest-path postgres_console_app/Cargo.toml
```

Примечания по `postgres_console_app`

- Возможно потребуется запущенный сервер PostgreSQL и переменные окружения (`DATABASE_URL` или другие), зависящие от реализации приложения. Проверьте `postgres_console_app/src/main.rs` для деталей.

Где искать скомпилированные бинарники

- Бинарники находятся в `target/debug/` (для debug) и `target/release/` (после сборки с `--release`). Например:

```bash
# debug-бинарник hello_world
./hello_world/target/debug/hello_world
```

Быстрая сборка всех проектов (в корне репозитория)

```bash
cargo build --manifest-path hello_world/Cargo.toml && \
cargo build --manifest-path guessing_game/Cargo.toml && \
cargo build --manifest-path postgres_console_app/Cargo.toml
```

Если хотите, могу:

- Закоммитить этот файл за вас.
- Добавить инструкции по настройке PostgreSQL (если сообщите, какие env-var нужны).

---  VPN Project 1.0 Strucrute:

vpn project/
├── .venv/                  # Python virtual environment folder
├── .gitignore              # Файл, чтобы не пушить мусор в Git
├── requirements.txt        # Список зависимостей Python
├── main.py                 # Главный файл интерфейса (GUI)
└── src/                    # Папка для кода на Rust
    ├── Cargo.toml          # Конфигурация Rust-проекта и зависимостей PyO3
    └── lib.rs              # Код ядра VPN на Rust
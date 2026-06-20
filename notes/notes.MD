---  VPN Project 1.0 Strucrute (notes for developer):

VPN project/
│
├── .venv/
├── target/
├── src/
│   └── lib.rs              
│
├── Cargo.lock
├── Cargo.toml              # Кфг Rust и описание связки PyO3
├── requirements.txt
├── .gitignore
│
├── main.py
├── config.json             # Кфг файл настроек прокси-маршрутов для Xray
│
├── xray.exe                # основной исполняемый бинарник ядра VPN
├── geoip.dat
└── geosite.dat
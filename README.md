# LE.GO.LAS Desktop (Tauri 2.0)

Нативное desktop-приложение для платформы **LE.GO.LAS** на базе [Tauri 2.0](https://tauri.app/).  
Оборачивает существующий Next.js фронтенд (`../frontend/steptwo_frontend`) в нативное окно Windows / macOS / Linux.

## Структура проекта

```
steptwo_desktop/
├── src-tauri/                  # Rust/Tauri backend
│   ├── src/
│   │   ├── main.rs             # Binary entry point
│   │   ├── lib.rs              # App setup (tray, shortcuts, plugins)
│   │   ├── eimzo.rs            # E-IMZO native WSS bridge (replaces eimzo-proxy.js)
│   │   ├── tray.rs             # System tray icon + context menu
│   │   └── shortcuts.rs        # Global hotkeys (Ctrl+Shift+N / F)
│   ├── icons/                  # App icons (generate with: cargo tauri icon <logo.png>)
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json         # Main app config
│   └── tauri.conf.kiosk.json   # Kiosk build for meeting room tablets
├── package.json
└── README.md
```

## Быстрый старт (dev-режим)

```powershell
# 1. Запустить Next.js dev server (в отдельном терминале)
cd ../frontend/steptwo_frontend
npm run dev

# 2. Запустить Tauri dev-окно (в папке steptwo_desktop)
npm run dev
# или: npx @tauri-apps/cli dev
```

При первом запуске Rust скомпилирует все зависимости (~3-5 минут).

## Иконки (обязательно перед сборкой)

```powershell
npx @tauri-apps/cli icon path/to/legolas-1024.png
# Сгенерирует все нужные размеры в src-tauri/icons/
```

## Сборка пакетов

```powershell
# Основная сборка (Windows: .exe + .msi, macOS: .dmg, Linux: .AppImage + .deb)
npm run build

# Kiosk-сборка для планшетов переговорных
npx @tauri-apps/cli build --config src-tauri/tauri.conf.kiosk.json
```

Артефакты: `src-tauri/target/release/bundle/`

## Нативные фичи

| Фича | Описание |
|------|----------|
| **E-IMZO Bridge** | Прямое нативное WSS подключение к `127.0.0.1:64443` без прокси |
| **System Tray** | Иконка в трее, закрытие окна = сворачивание (не выход) |
| **Global Hotkeys** | `Ctrl+Shift+N` — создание задачи, `Ctrl+Shift+F` — поиск |
| **OS Notifications** | Нативные уведомления при новых задачах / документах |
| **Kiosk Mode** | Полноэкранный режим для настенных планшетов переговорных |

## Нет Code Signing (для внутреннего тестирования)

- **Windows:** SmartScreen предупредит «Неизвестный издатель» → нажать «Подробнее» → «Выполнить»
- **macOS:** Gatekeeper заблокирует — открыть через ПКМ → «Открыть» → подтвердить
- **Linux:** AppImage — запустить через терминал: `chmod +x *.AppImage && ./*.AppImage`

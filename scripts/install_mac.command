#!/bin/bash
# ------------------------------------------------------------------
# LE.GO.LAS Desktop — Автоматический установщик для macOS
# ------------------------------------------------------------------

clear
echo "======================================================"
echo "    🚀 Установка приложения LE.GO.LAS на ваш Mac      "
echo "======================================================"
echo ""

# Определяем директорию скрипта
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
APP_NAME="LE.GO.LAS.app"

# Ищем .app файл рядом со скриптом или в родительской папке
if [ -d "$DIR/$APP_NAME" ]; then
    SOURCE_APP="$DIR/$APP_NAME"
elif [ -d "$DIR/../$APP_NAME" ]; then
    SOURCE_APP="$DIR/../$APP_NAME"
elif [ -d "/Volumes/LE.GO.LAS/$APP_NAME" ]; then
    SOURCE_APP="/Volumes/LE.GO.LAS/$APP_NAME"
else
    # Ищем любой .app в текущей директории
    SOURCE_APP=$(find "$DIR" -maxdepth 2 -name "*.app" | head -n 1)
fi

if [ -z "$SOURCE_APP" ] || [ ! -d "$SOURCE_APP" ]; then
    echo "❌ Ошибка: Файл $APP_NAME не найден рядом со скриптом."
    echo "Пожалуйста, распакуйте архив и запустите скрипт снова."
    read -p "Нажмите Enter для выхода..."
    exit 1
fi

echo "📦 1/3 Копирование в папку «Программы» (/Applications)..."
rm -rf "/Applications/$APP_NAME" 2>/dev/null
cp -R "$SOURCE_APP" /Applications/

echo "🔓 2/3 Снятие системного карантина macOS..."
xattr -cr "/Applications/$APP_NAME" 2>/dev/null

echo "✨ 3/3 Запуск LE.GO.LAS..."
open "/Applications/$APP_NAME"

echo ""
echo "======================================================"
echo "   ✅ Установка успешно завершена!"
echo "   Приложение LE.GO.LAS готово к работе."
echo "======================================================"
echo ""
sleep 2
exit 0

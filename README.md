<h1 align="center">  Esp-ioT 🗼 </h1>
<div align = "center">
<a href="https://t.me/ArcaneDevStudio" target="_blank" rel="noopener noreferrer">
    <img src="https://img.shields.io/badge/Telegram-Channel-blue?style=flat-square&logo=telegram" alt="Telegram">
</a>
<a href="https://t.me/Nam4iks" target="_blank" rel="noopener noreferrer">
    <img src="https://img.shields.io/badge/Telegram-Contact-blue?style=flat-square&logo=telegram" alt="Telegram">
</a>
<br>
<a href="https://github.com/Nam4ik/esp-ioT/actions", target="_blank", rel="noopener noreferrer">
    <img src="https://github.com/Nam4ik/esp-iot/actions/workflows/rust.yml/badge.svg?event=push", alt="rust.yml">
</a> 
<img src="https://img.shields.io/badge/Version-v0.1-blue.svg">
<a href="https://github.com/Nam4ik/esp-iot/blob/main/EN-README.md" target="_blank" rel="noopener noreferrer">
  <br>
  <img src="https://img.shields.io/badge/README-English-blue?style=flat-square&logo=github" alt="English README">
</a>
</div>
<br>

> [!WARNING]
> Programming is more of a hobby than a job, and there may be downtime in development.

## Описание 

> [!NOTE]
> Вы в ветке dev, код актуален

**Esp-ioT** - pet проект призванный создать встраиваемую систему для esp32 на раст (xtensa32).
<br>
Основная ветка - `dev`. В ней не рабочий код который в итоге переходит сливается сюда и собирается на странице релизов.
<br>

## Касаемо реализации протокола ioT
На esp32 запускается веб сервер в локальной сети (192.168.1.XX) и конфигурация проходит через него. Сам протокол первое время не будет полноценным, сможет управлять датчиками лидара, некоторыми датчиками температуры, влажности и т.п. 
<br>
Реализована будет библиотека для C/CXX и крейт для раст для дополниьельных функций и протоколов. Например экрана и других интеграций и автоматизаций если вам вдруг не хватит веб графики и системы событий.
<br>
Вообще в хотел бы добавить поддержку умного дома но увы - все проприетарное и врядли я смогу чтото сделать. Я конечно учусь на реверс инженера но на территории РФ все виды реверсивной инженерии запрщены, а еще добивает свободное время,
которого мало, особенно перед экзаменами. 

## Ход реализации

Dev branch
- [x] - Основная прошивка (минимум, без ioT)
- [ ] - Минимальный протокол ioT
- [x] - Web-сервер
- [ ] - WebUI для настройки

<br>

- [ ] - mvp профиль
- [ ] - максимальный профиль
<br>

Поддерживаемые датчики - bh1750, hcsr04, bmp280

## Сборка и прошивка
Для сборки бинарника вам потребуется: 
- Доступ в интернет
- Root права или установленные зависисмости
- Linux/BSD/WSL десктоп. Поддержка wsl эксперементальна.

Сборка: 
```shell
git clone https://github.com/Nam4ik/esp-iot/
cd esp-iot
sudo pacman -S espup
espup install
./setup.sh
```


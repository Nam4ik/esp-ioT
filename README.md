<h1 align="center">  Esp-ioT 🗼 </h1>
<div align = "center">
<a href="https://t.me/ArcaneDevStudio" target="_blank" rel="noopener noreferrer">
    <img src="https://img.shields.io/badge/Telegram-@ArcaneDevStudio-blue?style=flat-square&logo=telegram" alt="Telegram">
</a>
<a href="https://github.com/Nam4ik/esp-ioT/actions", target="_blank", rel="noopener noreferrer">
    <img src="https://github.com/Nam4ik/esp-iot/actions/workflows/rust.yml/badge.svg?event=push", alt="rust.yml">
</a> 
<img src="https://img.shields.io/badge/Version-v0.1-blue.svg">
</div>
<br>

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
- [ x ] (Основная прошивка)
- [   ] (Минимальный протокол ioT)
- [ x ] (Web-сервер)
- [   ] (WebUI для настройки)
Поддерживаемые датчики - bh1750, hcsr04, bmp280

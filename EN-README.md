<h1 align="center"> Esp-IoT 🗼 </h1>
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
<br>
</div>

## Description 

**Esp-IoT** is a pet project that is an integrated system for esp32 on a computer (xtensa32).
<br>
The main branch is dev. It does not contain working code, which eventually goes to merge here and is collected on the releases page.

## About ioT proto realisation
The esp32 runs a web server on the local network (192.168.1.XX) and the conguration runs through it. The protocol itself will not be complete for the first time, it will be able to control lidar sensors, some temperature, humidity
sensors, etc.
The biblioteka company was created for C/CXX and is intended for crop production, animal husbandry and processing industries. For example, the screen and other integrations and automations if you suddenly do not have enough web graphics and event system.
<br>
In general, I would like to add smart home support, but alas, everything is proprietary and I'm unlikely to be able to do anything. Of course, I'm studying to be a reverse engineer, but all types of reverse engineering are banned in the Russian Federation, and I also have a lot of free time,
which is not enough, especially before exams. 

## Implementation progress

Developer Branch
- [x] - Main load
- [ ] - Minimum IOP Protocol
- [x] - web server
- [ ] - Web interface for configuration

<br>

Supported sensors are BH1750, HCSR04, BMP280
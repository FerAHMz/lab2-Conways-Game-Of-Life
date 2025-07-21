# Conway's Game of Life — Laboratorio 2

Este proyecto consiste en una implementación **desde cero** del famoso autómata celular **Game of Life** de John Conway, utilizando **Rust** y la biblioteca gráfica **Raylib** para mostrar visualmente la evolución del sistema. 

---

## 🎯 Objetivos alcanzados

✅ Implementar una simulación funcional del Game of Life desde cero.  
✅ Modularización del proyecto utilizando buenas prácticas en Rust.  
✅ Implementación de **10 patrones** clásicos (Still lifes, Oscillators y Spaceships).  
✅ Adición de un **bonus creativo**: **Gosper Glider Gun**.  
✅ Renderizado eficiente usando un framebuffer personalizado.  
✅ Visualización clara en pantalla llena utilizando **escalado x6**.  
✅ Generación de una animación del resultado en `Lab2.gif`.

---

## 🧬 Patrones implementados

El proyecto incluye los siguientes **10 patrones clásicos** más un **bonus**:

### 🧊 Still lifes
- Block
- Beehive

### 🔄 Oscillators
- Blinker
- Beacon
- Toad
- Pulsar
- Pentadecathlon

### 🚀 Spaceships
- Glider
- LWSS (Light-weight Spaceship)
- MWSS (Middle-weight Spaceship)

### 🎁 BONUS
- **Gosper Glider Gun** (Generador de gliders infinito)

Estos patrones están distribuidos en la pantalla y escalados para cubrir una porción significativa del espacio.

---

## ▶️ Cómo ejecutar el proyecto

### 1. Requisitos

- Tener instalado [Rust](https://www.rust-lang.org/tools/install)
- Tener instalado Raylib (ya sea manualmente o vía bindings de `raylib-rs`)

### 2. Clona el repositorio

```bash
git clone https://github.com/tu_usuario/lab2-Conways-Game-Of-Life.git
cd lab2-Conways-Game-Of-Life/lab2
```
3. Ejecuta el proyecto
```bash
cargo run
```
Esto abrirá una ventana con la simulación en tiempo real de Conway's Game of Life.

## 📸 Resultado final
![Resultado final](lab2/Lab2.gif)

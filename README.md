# Hispano - Lenguaje de Programación

**Hispano** es un lenguaje transpilado de programación con sintaxis en español, pensado para hacer la programación más accesible y natural para hablantes de español.

- El repositorio de GitHub: [Hispano en GitHub](https://github.com/xaviza11/Hispano)

- Encuentra su extensión para VS Code aquí: [Hispano VS Code Extension](https://github.com/xaviza11/Hispano-vsc-extension)


## ✨ Características 
- **Sintaxis en español**: Palabras clave como `funcion`, `estructura`, `var`, `si`, etc.
- **Tipado estático y orientación a objetos**.
- **Integración con Rust**: Hispano transpila su codigo a rust, aprovecha características de Rust.
---

## 💻 Ejemplos de código 
- **Hispano** se escribe en la carpeta `app`, por defecto encontrarás algunos ejemplos básicos de cómo usarlo. La palabra -  **main**  - en Hispano se escribe como - **jefe** -, en `/app` encontrarás el archivo [**jefe.his**](./app/jefe.his).
---


## ⚙️ Instalación 
Para instalar **Hispano**, sigue los siguientes pasos:

1. En la **raíz del proyecto**, construye el proyecto con el siguiente comando:
   ```bash
   cargo build
   ```
2. Luego, **instala Hispano** de forma global con el siguiente comando:
      ```bash
    cargo install --path .
    ```

- Esto te permite **ejecutar Hispano** desde la línea de comandos.
---

## 📜 Comandos disponibles 

- **Una vez instalado**, puedes utilizar los siguientes comandos con Hispano:

```bash
hispano construir    # Construye la aplicación.
hispano correr       # Corre la aplicación.
hispano alzar        # Construye y corre la aplicación.
hispano test         # Ejecuta los tests del proyecto.
hispano comandos     # Muestra esta lista de comandos.
hispano --version    # Muestra la versión de Hispano.
```
---

##  📚 Palabras clave
- Para ver las **palabras clave** accede a [**DICCIONARIO.md**](./DICCIONARIO.md)
---

## 🐞 Problemas Conocidos
- Si encuentras algún problema, por favor abre un **issue** en el repositorio.
---

## 📝 Notas de la versión
### 1.0.0
- Lanzamiento inicial con resaltado de sintaxis básico en español.
---

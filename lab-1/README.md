# Polygon Filler - Laboratorio 1

Este proyecto implementa un algoritmo de relleno de polígonos usando el método Scanline en Rust con Raylib.

## Características

- **Algoritmo Scanline**: Implementa el algoritmo de relleno por líneas de barrido
- **Detección de agujeros**: Puede identificar y manejar agujeros dentro de polígonos
- **Múltiples polígonos**: Dibuja 5 polígonos diferentes con colores únicos
- **Framebuffer personalizado**: Implementa un framebuffer para control preciso de píxeles
- **Exportación de imagen**: Genera un archivo `out.png` con el resultado

## Polígonos incluidos

1. **Polígono 1** (Rojo): 10 puntos - (165, 380) (185, 360) (180, 330) (207, 345) (233, 330) (230, 360) (250, 380) (220, 385) (205, 410) (193, 383)
2. **Polígono 2** (Verde): 4 puntos - (321, 335) (288, 286) (339, 251) (374, 302)
3. **Polígono 3** (Azul): 3 puntos - (377, 249) (411, 197) (436, 249)
4. **Polígono 4** (Amarillo): 18 puntos - Forma compleja con agujero
5. **Polígono 5** (Agujero): 4 puntos - (682, 175) (708, 120) (735, 148) (739, 170) - Agujero dentro del polígono 4

## Requisitos

- Rust (versión 1.70 o superior)
- Cargo

## Instalación y ejecución

1. Clona el repositorio:
```bash
git clone <tu-repositorio>
cd lab-1
```

2. Compila y ejecuta el proyecto:
```bash
cargo run
```

3. El programa mostrará una ventana con los polígonos dibujados y generará un archivo `out.png` en el directorio raíz.

## Estructura del código

- `src/main.rs`: Contiene toda la lógica del programa
- `Cargo.toml`: Configuración del proyecto y dependencias
- `out.png`: Imagen generada con los polígonos rellenados

## Algoritmo implementado

### Scanline Algorithm
El algoritmo de líneas de barrido funciona de la siguiente manera:

1. Para cada línea horizontal (scanline) desde el mínimo Y hasta el máximo Y del polígono
2. Encuentra todas las intersecciones de la línea con los bordes del polígono
3. Ordena las intersecciones por coordenada X
4. Rellena los píxeles entre pares de intersecciones
5. Para cada píxel a rellenar, verifica si está dentro de algún agujero
6. Solo rellena si el píxel no está dentro de un agujero

### Detección de agujeros
Se utiliza el algoritmo de ray casting (point-in-polygon) para determinar si un punto está dentro de un agujero:
- Traza una línea horizontal desde el punto hacia la derecha
- Cuenta el número de intersecciones con los bordes del agujero
- Si el número es impar, el punto está dentro del agujero

## Controles

- **ESC**: Salir del programa
- La ventana se puede cerrar normalmente

## Salida

El programa genera:
- Una ventana interactiva mostrando los polígonos
- Un archivo `out.png` con la imagen final de los polígonos rellenados

## Notas técnicas

- El framebuffer se implementa como un vector de colores
- Se usa el algoritmo de Bresenham para dibujar las líneas de borde
- Los colores están predefinidos para cada polígono
- El agujero se detecta automáticamente y no se rellena 
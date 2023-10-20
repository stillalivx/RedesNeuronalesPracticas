# Redes neuronales en [Rust](https://www.rust-lang.org/)

## Instalación

Para ejecutar este proyecto, se necesita tener Rust instalado en el equipo. Si no lo tienes, puedes seguir los pasos de la [guía de instalación de Rust](https://www.rust-lang.org/tools/install).

## Ejecución

Compila y ejecuta con una sola línea gracias a ```cargo```. 

```shell
cargo run
```

Es necesario el uso de banderas para escoger la red neuronal que se desea usar. A continuación se muestran las banderas disponibles con su respectiva red neuronal.

| **Flag**   | **Red neuronal**                                           |
|------------|------------------------------------------------------------|
| --hopfield | [Hopfield](https://en.wikipedia.org/wiki/Hopfield_network) |
| --hamming  | Hamming                                                    |

Una vez escogida la red neuronal que se desea ejecutar, agregar la bandera al comando ```cargo run```:

```shell
cargo run -- --hopfield
```

## Opciones adicionales

| **Flag**             | **Descripción**                                                                                                 |
|----------------------|-----------------------------------------------------------------------------------------------------------------|
| --show-weight-matrix | Muestra en consola la matriz de peso generada durante la ejecución                                              |
| -i                   | Habilita la opción de recibir imágenes como entrada. Esperando la ruta de la imágen como entrada en cada patrón |
| --idir ruta_dir      | Recibe como argumento la ruta de una carpeta de imágenes que se deseen insertar como entrada de la red neuronal |





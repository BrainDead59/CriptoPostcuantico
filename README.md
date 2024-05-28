# Algoritmos criptográficos postcuánticos:
En este proyecto realizamos la implementación de los algoritmos postcuánticos autorizados por el NIST, el en lenguaje de programación Rust:
- Kyber
- Dilithium
- Sphincs+

## Integrantes
- Díaz Hernández Marcos Bryan
- Fernández Rosales Sebastián
- Medina Segura Fernando 
- Robledo Aguirre Eduardo 
- Toledo Sánchez Roberto

## Instalar Rust
Este proyecto los ejecutamos en un entorno linux por lo que la configuracíon siguiente es para cualquier distribución de linux, unicamente será necesario adaptar los comandos a su distribución.

Antes de instalar Rust es necesario instalar cmake y curl para poder hacer una compilación sin problemas. Para instalar Rust se utiliza el siguiente comando en la terminal:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Se debe de continuar con el instalador en la terminal y realizar la configuración del sistema para que detecte el lenguaje:


```shell
source $HOME/.cargo/env
source $HOME/.bashrc 
source $HOME/.profile
```

Finalmente se puede comprobar la instalación con el comando:

```shell
rustc --version
cargo --version
```

## Compilar el proyecto

Al tener Rust instalado se tiene que descargar el proyecto y antes de proceder a la compilación se debe de  instalar el compilador nightly de rust, esto porque las bibliotecas de donde se tiene la implementación de los algoritmos están en desarrollo. [Más informacion.](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)

Para descargar la versión nightly de rust se utiliza el siguiente comando:

```shell
rustup install nightly
```

Posteriormente se necesita realizar el cambio de versión stable a la versión nightly:

```shell
rustup default nightly
```

Finalmente para compilar el proyecto se usa el siguiente comando:

```shell
cargo run
```


## Referencias:
- [Proyecto de Argyle Software.](https://github.com/Argyle-Software)
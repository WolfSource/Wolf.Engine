# Wolf Engine [![LGPL v3 licensed](https://img.shields.io/badge/license-Apache-blue)](https://github.com/WolfEngine/Wolf.Engine/blob/main/LICENSE.md) [![wakatime](https://wakatime.com/badge/github/WolfEngine/WolfEngine.svg)](https://wakatime.com/badge/github/WolfEngine/WolfEngine)
<img src="https://raw.githubusercontent.com/WolfEngine/WolfEngine/main/Logo.png" width="256" height="256" alt="WolfEngine"/>

**Wolf Engine** is the next generation of [Persian Game Engine](https://github.com/PooyaEimandar/PersianEngine) which is a
cross-platform open source game engine created by [Pooya Eimandar](https://pooyaeimandar.github.io)
This Wolf is a comprehensive set of Rust/C libraries for realtime rendering, realtime streaming and game developing, which is support **Lua** & **Python** as an embedded script and binding language.</p>

## Projects using Wolf
- [Wolf.Playout](https://www.youtube.com/watch?v=EZSdEjBvuGY) is a playout automation software
- [Falcon](https://youtu.be/ygpz35ddZ_4) is a real time 3D monitoring system, developed at [FANAP Co.](https://fanap.ir/)
- Barf is a real time streaming core for cloud gaming platform

## Branches
- [main](https://github.com/WolfEngine/WolfEngine/tree/main), Wolf3, is the latest version of Wolf which is written in **Rust and contains some unsafe codes** and is not ready for production
- [Wolf2](https://github.com/WolfEngine/WolfEngine/tree/wolf-2) is written in **C/C++ and is in maintenance mode**
- [releases](https://github.com/WolfEngine/WolfEngine/releases) contains old releases and source codes

## Build
- **Wolf 2/1** via CMake
- **Wolf 3**
  First install the command-line bindgen tool for linking Wolf/CC dependencies
  ```bash
  cargo install bindgen
  ```
  
  - For **Webassembly** :\
  From WolfEngine folder
  ```bash
  rustup default nightly
  rustup target add wasm32-unknown-unknown
  cd wolf-demo
  ./build-wasm.sh
  ./run-wasm.sh
  ```
  Finally the demo will be served at http://localhost:8000
  - For **Windows, MacOS, Linux** :
  ```bash
  rustup default stable
  cd wolf-demo
  cargo run
  ```
  - For **Android** :
  ```bash
  rustup default stable
  rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
  cargo install cargo-ndk
  cd wolf
  cargo ndk -t armeabi-v7a -t arm64-v8a -o ./jniLibs build --release 
  ```

  - For **iOS** :
  ```bash
  rustup default stable
  rustup target add \
    aarch64-apple-ios \
    x86_64-apple-ios
  cargo install cargo-lipo
  cd wolf
  cargo lipo --release
  ```

## Copyright & License
Wolf Engine © 2014-2022 [Pooya Eimandar](https://www.linkedin.com/in/pooyaeimandar)
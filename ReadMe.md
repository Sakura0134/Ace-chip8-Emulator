# ACE(Another Chip8 Emulator)
<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/Sakura0134/Ace-chip8-Emulator/main/assets/ace2.png" alt="ace" width="400">
  <img src="https://raw.githubusercontent.com/Sakura0134/Ace-chip8-Emulator/main/assets/ace.png" alt="ace" width="400">
  <br>
  <b>ACE(Another Chip8 Emulator)</b>
  <br>
</h1>
This is a chip8 emulator created using Rust programming Language. It's purpose is to learn Rust at the same time, graphic API's and GUI libraries. I am planning to add JIT, Vulkan support and WEBGPU support in the future.

## Setup Emulator
1. Download the emulator
2. Unzip it
3. Edit config/config.ini
4. Launch the emulator from the folder with arguments `./ace_cli [GAME LOC]`\
You can also change the config file location and execute it using `./ace-cli [GAME LOC] [CONFIG LOC]`

## Things to be Done
- [x] Boot Games
- [x] Add OpenGL Backend
- [ ] Add Audio Support
- [x] Add Keyboard Support
- [x] Emulate Chip8 Hardware
- [ ] Add Vulkan Backend
- [ ] Add wgpu Backend
- [ ] Add jit recompiler
- [ ] Add a intermediate GUI library

## Imperfections in Code
- [ ] Need to find a better way to handle inputs
- [ ] Optimize opengl code

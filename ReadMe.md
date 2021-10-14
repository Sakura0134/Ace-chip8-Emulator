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

## Config.ini
It handles basic settings in code. It handles basic settings in code. Run the emulator once to create it.(In config folder in project repo)
```
[Theme]
bg = 3 //Sets background Colour
fg = 4 //Sets Foreground Colour

[Hack]
delay = 100 //Sets delay
clock = 700 //Sets clock speed(700 is the optimal. Recommended range 500 - 1000)

[Screen]
scale = 10 //Sets scaling

[Audio]
enable = true //Sets Audio

[Shader] //To Specify Shader Location
v_shader = config/shader/triangle.vert
f_shader = config/shader/triangle.frag
```

0. Red 1. Green 2. Blue 3. Black 4. White 5. Light Pink 6. Light Blue 7. Blue Violet 8. Yellow 9. Lawn Green


## Things to be Done
- [x] Boot Games
- [x] Add OpenGL Backend
- [x] Add Audio Support
- [x] Add Keyboard Support
- [x] Emulate VM
- [ ] Add Vulkan Backend
- [ ] Add wgpu Backend
- [ ] Add jit recompiler
- [ ] Add a intermediate GUI library

## Imperfections in Code
- [ ] Find the bad instruction(causing troubles in INVADERS)
- [ ] Hint the bug that causes stack overflow in Invaders

# Todo

## 1. Setup basic premake5.lua generation

All generated files will be in the `.craft/` directory inside the project directory. Premake can then be called with `premake5 --file=.craft/premake5.lua`. This will automatically place all generated files inside the `.craft/` folder, meaning the root of the project directory is kep clean, where the source files and Craft config file can go. `gmake2` is prefered for \*nix, while the latest Visual Studio will be the default for Windows.

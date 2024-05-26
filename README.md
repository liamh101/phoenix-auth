# Phoenix Auth

A free Open Source desktop 2FA Application.

## Features

- 6/8 Digit Support
- 30/60/90/120 Token Refresh Rate
- SHA1/SHA256/SHA512 Algorithm
- WA import support
- Linux, Mac and Windows Support

## Roadmap

- Account Colour Picker
- Dark Mode
- Folder Organisation
- External Account Backups and Syncing
- Additional Import Support
- Mobile Support

## Contribution

### Tools

- [Tauri V1](https://tauri.app/)
- [Vue 3](https://vuejs.org/)

### Recommended IDE Setups

- [RustRover](https://www.jetbrains.com/rust/)
- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).

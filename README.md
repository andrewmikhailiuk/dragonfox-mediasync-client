# DragonFox MediaSync Client

Desktop client for syncing media play/pause across devices via WebSocket.

## Features

- Global hotkey to toggle play/pause (default: Cmd+Shift+Space)
- Sends toggle command to server for sync with other clients
- Simulates system media key locally so player responds
- System tray with connection status
- Auto-reconnect with exponential backoff

## Tech Stack

- **Frontend:** Vue 3 + TypeScript + Vite + Pinia
- **Backend:** Tauri 2.x (Rust)
- **Styling:** Tailwind CSS

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Platform Support

| Platform | Media Key Simulation |
|----------|---------------------|
| macOS    | NSEvent + CGEventPost |
| Windows  | SendInput API |

## Usage

1. Start the server
2. Launch the client
3. Enter server URL and room name
4. Set a global hotkey in Settings
5. Press hotkey to sync play/pause across all connected clients

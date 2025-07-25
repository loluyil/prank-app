# Jumpscare App

Application build on Tauri v2

## Features

### Jump Scares
- Foxy Jumpscare !!! LOUD
- Rigby fade in - out

###  Cursor Control
- Cursor moves to left
- Cursor goes to the corners of screen

### Audio Integration
- Discord notification
- Knocking sound

###  Terminate program
- Ctrl + Alt + P to close the program

### TO DO
- Need to make click event on cursor to move around whenever left-click is pressed
- GUI(opened w/ Hotkey) to test/enable/disable features

---

## Installation

```bash
git clone [repository]
npm install
npm run tauri dev
```

## Usage

```javascript
// Trigger popup scare
await invoke('toggle_popup_window');

// Move cursor smoothly
await invoke('cursor_move', { 
  target_x: 500, 
  target_y: 300, 
  duration_secs: 2.0 
});

// Navigate corners
await invoke('cursor_corners');

// Get mouse position
const pos = await invoke('grab_mouse_position');
```

##  Controls

| Key Combination | Action |
|-----------------|--------|
| `Ctrl + Alt + P` | Force quit application |

##  Technical Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: JavaScript/HTML  
- **System APIs**: Windows API for native functionality
- **Features**: Global hotkeys, window management, cursor control

---

##  Disclaimer

just for jokes, install on your friend's pc

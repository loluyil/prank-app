# 👻 Jumpscare App

A desktop application built with Tauri v2 for creating jump scares and controlling system interactions. Features popup windows, cursor manipulation, and audio effects.

## ✨ Features

### 🎯 Jump Scares
- Popup windows with fade effects
- Timed display sequences  
- Hidden window operation

### 🖱️ Cursor Control
- Smooth mouse movement with easing
- Screen corner navigation
- Position tracking

### 🔊 Audio Integration
- Sound effect playback
- Synchronized audio/visual scares

### ⚙️ System Integration
- Global hotkeys (Ctrl+Alt+P to terminate)
- Background operation
- Native Windows API integration

---

## 🚀 Installation

```bash
git clone [repository]
npm install
npm run tauri dev
```

## 📖 Usage

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

## ⌨️ Controls

| Key Combination | Action |
|-----------------|--------|
| `Ctrl + Alt + P` | Force quit application |
| `F12` | Toggle developer tools |

## 🛠️ Technical Stack

- **Backend**: Rust with Tauri v2
- **Frontend**: JavaScript/HTML  
- **System APIs**: Windows API for native functionality
- **Features**: Global hotkeys, window management, cursor control

---

## ⚠️ Disclaimer

Intended for educational purposes and harmless pranks with consent. Users are responsible for ethical usage.

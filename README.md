<div align="center">
  <img src="src-tauri/icons/128x128.png" width="96" alt="Gloss">
  <h1>Gloss</h1>
  <p>Select text in any app, press a hotkey, read the translation.</p>
</div>

Gloss is a small tray app that translates whatever you have selected, right next to your cursor. It works in any application,
with the AI provider of your choice, cloud or local.

## Features

- **Works everywhere**: browsers, PDF readers, chat apps, editors. Select, press `Alt+O`, done.
- **Translate your writing**: select what you typed and press `Alt+P`. Gloss translates it into your chosen language and
  replaces your text, after a preview or straight away.
- **Quick language switch**: pick either language from the tray menu.
- **Any AI provider**: OpenAI, Anthropic, Google Gemini, and any OpenAI-compatible API such as OpenRouter, Groq, DeepSeek,
  Ollama or LM Studio.
- **Smart direction**: text already in your target language is translated into a fallback language instead.
- **Your style**: keeps the original tone by default; switch to formal, casual, literal, concise or learner notes, add custom
  instructions, or write your own system prompt.
- **Flexible popup**: open at the cursor, above it, centered, top right or where you last left it. Adjustable size, font and
  light or dark theme.
- **Stays out of the way**: lives in the tray, closes when you click elsewhere, can copy the result automatically and launch
  at login.

## Getting started

1. Install Gloss and launch it. It appears in the system tray.
2. Left click the tray icon to open Settings, pick a provider and enter your API key.
3. Select some text anywhere and press `Alt+O`. To translate your own text, select it and press `Alt+P`.

On macOS, grant the Accessibility permission when asked, then relaunch.

## Building from source

Requires [Node.js](https://nodejs.org) and [Rust](https://rustup.rs).

```
npm install
npm run tauri dev      # develop
npm run tauri build    # installer in src-tauri/target/release/bundle
```

## Notes

- The API key is stored in plain text in the app data folder.
- Tested on Windows. macOS support is written but not yet tested.

<div align="center">
  <img src="src-tauri/icons/128x128.png" width="112" alt="Gloss logo">

  <h1>Gloss</h1>

  <p><strong>Select text in any app. Press a hotkey. Read the translation, or learn what it is.</strong></p>

  <p>
    <img alt="Version" src="https://img.shields.io/github/v/release/meokisama/gloss?style=flat-square&color=6366f1">
    <img alt="Platform" src="https://img.shields.io/badge/platform-Windows-0ea5e9?style=flat-square">
    <img alt="Tauri" src="https://img.shields.io/badge/Tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white">
    <img alt="React" src="https://img.shields.io/badge/React-19-61dafb?style=flat-square&logo=react&logoColor=black">
    <img alt="Rust" src="https://img.shields.io/badge/Rust-stable-dea584?style=flat-square&logo=rust&logoColor=black">
  </p>

  <p>
    <a href="#features">Features</a> ·
    <a href="#getting-started">Getting started</a> ·
    <a href="#providers">Providers</a> ·
    <a href="#building-from-source">Build</a>
  </p>
</div>

<br>

Gloss is a tiny tray app that translates whatever you have selected and shows it right next to your cursor. It also works the
other way: select what you just wrote, and Gloss replaces it with the translation. Or select a term you do not know, and Gloss
explains what it is and how it works. Bring any AI provider, cloud or local.

## Three shortcuts

|    Shortcut    | What it does                                                                                                |
| :------------: | ----------------------------------------------------------------------------------------------------------- |
| `Ctrl+Shift+O` | **Read.** Translates the selection into your language and shows it in a popup at the cursor.                |
| `Ctrl+Shift+P` | **Write.** Translates your own text into the target language and pastes it back, with or without a preview. |
| `Ctrl+Shift+I` | **Explain.** Explains a term, concept, product or acronym, or gives a dictionary entry for an everyday word. |

All shortcuts are configurable. Read and Write each have their own language and style that you can switch from the tray menu.
Explain answers in the Read language.

## Features

- **Works everywhere.** Browsers, PDF readers, chat apps, editors: if you can select it, Gloss can translate or explain it.
- **Keeps your voice.** Matches the original tone by default. Switch to Formal, Casual or Chatting, add custom
  instructions, or write your own system prompt.
- **Keeps the intent.** A question stays a question and a request stays a request; Gloss translates, it does not answer.
- **Explains, not just defines.** Says what the thing is, why it exists and how it works step by step, with real examples
  and every technical term spelled out.
- **Learns words with you.** Select an everyday word or phrase and Explain gives its pronunciation, meanings, when to use it
  and example sentences with translations.
- **Your popup, your way.** Open at the cursor, above it, centered, top right, or where you last left it. Adjustable width,
  height, font size and light or dark theme.
- **Stays out of the way.** Lives in the tray, closes when you click elsewhere, can copy the result automatically and launch at
  login.

## Getting started

1. Install Gloss and launch it. It appears in the system tray.
2. Left click the tray icon to open Settings, pick a provider, enter your API key and choose a model.
3. Select some text anywhere and press `Ctrl+Shift+O`. To translate your own writing, select it and press `Ctrl+Shift+P`. To
   look up what something is, select it and press `Ctrl+Shift+I`.

## Providers

| Provider              | Examples                                                             |
| --------------------- | -------------------------------------------------------------------- |
| OpenAI                | GPT models                                                           |
| Anthropic             | Claude models, with adjustable thinking effort                       |
| Google Gemini         | Gemini models                                                        |
| OpenAI-compatible API | OpenRouter, Groq, DeepSeek, and local models via Ollama or LM Studio |

Models are listed straight from the provider, so new ones show up without an update.

## Building from source

Requires [Node.js](https://nodejs.org) and [Rust](https://rustup.rs).

```sh
npm install
npm run tauri dev      # develop
npm run tauri build    # installer in src-tauri/target/release/bundle
```

Built with [Tauri 2](https://tauri.app), React 19, Tailwind CSS 4 and Rust.

## Releasing

Run `npm run release:patch` (or `release:minor`, `release:major`) on a clean tree. It bumps `package.json`, which is the app
version, commits, tags and pushes. GitHub Actions builds Windows, publishes the release and its `latest.json`. Apps install an update
quietly at launch. While running they check every 6 hours, download in the background and install on "Quit Gloss", or at once
from "Restart to update" in the tray menu.

Updates are signed. `plugins.updater.pubkey` in `tauri.conf.json` holds the public key; the repo secrets
`TAURI_SIGNING_PRIVATE_KEY` and `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` hold the private key. Losing it means installed apps can
no longer update.

## Known limitations

- The API key is stored in plain text in the app data folder.
- Windows only for now. macOS support is written but not tested, and releases do not build it.
- Explain answers from what the model knows; it does not search the web, so recent things may be missing or out of date.

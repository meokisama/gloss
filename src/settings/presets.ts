import type { Settings } from "@/shared/settings";

type ProviderPreset = Pick<Settings, "apiFormat" | "baseUrl" | "model"> & { name: string };

export const PROVIDER_PRESETS: ProviderPreset[] = [
  { name: "OpenAI", apiFormat: "openai", baseUrl: "https://api.openai.com/v1", model: "gpt-6-luna" },
  { name: "Anthropic", apiFormat: "anthropic", baseUrl: "https://api.anthropic.com", model: "claude-opus-5-5" },
  { name: "Google Gemini", apiFormat: "gemini", baseUrl: "https://generativelanguage.googleapis.com", model: "gemini-3.8-flash" },
  { name: "OpenRouter", apiFormat: "openai", baseUrl: "https://openrouter.ai/api/v1", model: "openai/gpt-5-mini" },
  { name: "Groq", apiFormat: "openai", baseUrl: "https://api.groq.com/openai/v1", model: "llama-3.3-70b-versatile" },
  { name: "DeepSeek", apiFormat: "openai", baseUrl: "https://api.deepseek.com/v1", model: "deepseek-chat" },
  { name: "Ollama (local)", apiFormat: "openai", baseUrl: "http://localhost:11434/v1", model: "qwen2.5:7b" },
  { name: "LM Studio (local)", apiFormat: "openai", baseUrl: "http://localhost:1234/v1", model: "local-model" },
];

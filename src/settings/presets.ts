import type { Settings } from "@/shared/settings";

type ProviderPreset = Pick<Settings, "provider" | "baseUrl" | "model"> & { name: string };

export const PROVIDER_PRESETS: ProviderPreset[] = [
  { name: "OpenAI", provider: "openai", baseUrl: "https://api.openai.com/v1", model: "gpt-5-mini" },
  { name: "Anthropic", provider: "anthropic", baseUrl: "https://api.anthropic.com", model: "claude-opus-5-5" },
  { name: "Google Gemini", provider: "gemini", baseUrl: "https://generativelanguage.googleapis.com", model: "gemini-2.5-flash" },
  { name: "OpenRouter", provider: "openai", baseUrl: "https://openrouter.ai/api/v1", model: "openai/gpt-5-mini" },
  { name: "Groq", provider: "openai", baseUrl: "https://api.groq.com/openai/v1", model: "llama-3.3-70b-versatile" },
  { name: "DeepSeek", provider: "openai", baseUrl: "https://api.deepseek.com/v1", model: "deepseek-chat" },
  { name: "Ollama (local)", provider: "openai", baseUrl: "http://localhost:11434/v1", model: "qwen2.5:7b" },
  { name: "LM Studio (local)", provider: "openai", baseUrl: "http://localhost:1234/v1", model: "local-model" },
];

export const LANGUAGES = [
  "Vietnamese",
  "English",
  "Japanese",
  "Chinese (Simplified)",
  "Chinese (Traditional)",
  "Korean",
  "French",
  "German",
  "Spanish",
  "Portuguese",
  "Russian",
  "Thai",
  "Indonesian",
];

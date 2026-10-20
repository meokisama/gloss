import { Languages, PanelTop, Server, Settings2 } from "lucide-react";
import { Tabs } from "radix-ui";
import type { FormEvent } from "react";

import { Footer } from "@/settings/components/footer";
import { Nav } from "@/settings/components/nav";
import { TitleBar } from "@/settings/components/title-bar";
import { GeneralSection } from "@/settings/sections/general-section";
import { PopupSection } from "@/settings/sections/popup-section";
import { ProviderSection } from "@/settings/sections/provider-section";
import { TranslationSection } from "@/settings/sections/translation-section";
import { SettingsContext } from "@/settings/settings-context";
import { useSettingsForm } from "@/settings/use-settings-form";
import { useTheme } from "@/shared/use-theme";

const PAGES = [
  { id: "general", label: "General", icon: Settings2, Content: GeneralSection },
  { id: "provider", label: "Provider", icon: Server, Content: ProviderSection },
  { id: "translation", label: "Translation", icon: Languages, Content: TranslationSection },
  { id: "popup", label: "Popup", icon: PanelTop, Content: PopupSection },
];

export function App() {
  const { settings, styles, status, update, save, test } = useSettingsForm();
  useTheme(settings?.theme);

  const onSubmit = (e: FormEvent) => {
    e.preventDefault();
    save();
  };

  return (
    <div className="flex h-screen flex-col">
      <TitleBar />
      {settings && (
        <SettingsContext.Provider value={{ settings, styles, update }}>
          <Tabs.Root defaultValue="general" orientation="vertical" asChild>
            <form className="flex min-h-0 flex-1" onSubmit={onSubmit}>
              <Nav pages={PAGES} />
              <main className="flex min-w-0 flex-1 flex-col rounded-tl-xl border-l border-t border-line bg-surface">
                <div className="flex-1 overflow-y-auto [scrollbar-gutter:stable]">
                  {PAGES.map(({ id, Content }) => (
                    <Tabs.Content key={id} value={id} className="outline-none">
                      <Content />
                    </Tabs.Content>
                  ))}
                </div>
                <Footer status={status} onTest={test} />
              </main>
            </form>
          </Tabs.Root>
        </SettingsContext.Provider>
      )}
    </div>
  );
}

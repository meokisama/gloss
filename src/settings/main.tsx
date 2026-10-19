import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "@/shared/theme.css";
import { App } from "@/settings/app";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);

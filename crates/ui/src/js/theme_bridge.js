export async function js_is_dark_mode() {
  const theme = document.documentElement.getAttribute("data-theme");
  if (theme === "dark") {
    return true;
  }
  if (theme === "light") {
    return false;
  }
  return !!(
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  );
}

export async function js_apply_theme(theme) {
  if (theme === "dark" || theme === "light") {
    document.documentElement.setAttribute("data-theme", theme);
    return;
  }
  const isDark = !!(
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  );
  document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
}

export async function js_toggle_theme() {
  const current = document.documentElement.getAttribute("data-theme");
  const next = current === "dark" ? "light" : "dark";
  document.documentElement.setAttribute("data-theme", next);
  return next;
}

export async function js_apply_grid_theme(theme) {
  if (!theme) {
    document.documentElement.removeAttribute("data-grid-theme");
    return;
  }
  document.documentElement.setAttribute("data-grid-theme", theme);
}

export async function js_apply_lang(lang) {
  if (!lang) {
    document.documentElement.removeAttribute("lang");
    return;
  }
  document.documentElement.setAttribute("lang", lang);
}

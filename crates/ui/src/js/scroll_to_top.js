function update_scroll_to_top_visibility() {
  const btn = document.getElementById("root-scroll-to-top");
  if (!btn) {
    return;
  }

  const scrollTop = window.scrollY || document.documentElement.scrollTop || 0;
  const visible = scrollTop > 320;

  btn.style.opacity = visible ? "1" : "0";
  btn.style.transform = visible ? "translateY(0)" : "translateY(8px)";
  btn.style.pointerEvents = visible ? "auto" : "none";
  btn.setAttribute("aria-hidden", visible ? "false" : "true");
}

function on_scroll_to_top_click() {
  window.scrollTo({ top: 0, behavior: "smooth" });
}

export async function mount_scroll_to_top(drop) {
  const btn = document.getElementById("root-scroll-to-top");
  if (!btn) {
    return;
  }

  update_scroll_to_top_visibility();
  window.addEventListener("scroll", update_scroll_to_top_visibility, { passive: true });
  window.addEventListener("resize", update_scroll_to_top_visibility);
  btn.addEventListener("click", on_scroll_to_top_click);

  drop.then(() => {
    window.removeEventListener("scroll", update_scroll_to_top_visibility);
    window.removeEventListener("resize", update_scroll_to_top_visibility);
    btn.removeEventListener("click", on_scroll_to_top_click);
  });
}

function update_scroll_progress() {
  const bar = document.getElementById("root-scroll-progress");
  if (!bar) {
    return;
  }

  const doc = document.documentElement;
  const scrollTop = window.scrollY || doc.scrollTop || 0;
  const scrollableHeight = Math.max(doc.scrollHeight - window.innerHeight, 0);
  const ratio = scrollableHeight > 0 ? scrollTop / scrollableHeight : 0;
  const progress = Math.min(100, Math.max(0, ratio * 100));
  bar.value = progress;
  bar.setAttribute("aria-valuenow", `${Math.round(progress)}`);
}

export async function mount_scroll_progress(drop) {
  update_scroll_progress();
  window.addEventListener("scroll", update_scroll_progress, { passive: true });
  window.addEventListener("resize", update_scroll_progress);

  drop.then(() => {
    window.removeEventListener("scroll", update_scroll_progress);
    window.removeEventListener("resize", update_scroll_progress);
  });
}

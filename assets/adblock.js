// Lightweight built-in cosmetic ad blocker. Not a Chrome extension or full network blocker.
(() => {
  if (window.__minimalAdBlockInstalled) return;
  window.__minimalAdBlockInstalled = true;
  const selectors = [
    'ins.adsbygoogle', '[data-ad-slot]', '[data-ad-client]',
    '[id^="google_ads_"]', '[id^="div-gpt-ad"]',
    '.adsbygoogle', '.advertisement', '.ad-banner',
    'iframe[src*="doubleclick.net"]', 'iframe[src*="googlesyndication.com"]',
    'iframe[src*="googleadservices.com"]'
  ];
  const css = selectors.map(s => `${s}{display:none!important;visibility:hidden!important}`).join('\n');
  const install = () => {
    if (document.getElementById('__minimal_adblock_style')) return;
    const style = document.createElement('style');
    style.id = '__minimal_adblock_style'; style.textContent = css;
    (document.head || document.documentElement).appendChild(style);
  };
  install();
  document.addEventListener('DOMContentLoaded', install, {once:true});
})();

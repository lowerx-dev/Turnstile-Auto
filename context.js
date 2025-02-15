function getOpenOrClosedShadowRoot(element) {
    if (chrome && chrome.dom && chrome.dom.openOrClosedShadowRoot) {
        return chrome.dom.openOrClosedShadowRoot(element);
    }
    return null;
}

async function main() {
    if (location.href.includes("challenges.cloudflare.com/cdn-cgi/challenge-platform/")) {
        const WASM_MOD_URL = chrome.runtime.getURL('turnstile_core.js');
        const wasm = await import(WASM_MOD_URL);

        await wasm.default().catch(() => {
            return null;
        });
        wasm?.inject();
    }
}

main();
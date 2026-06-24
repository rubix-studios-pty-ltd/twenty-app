pub fn inject_link(webview: &tauri::Webview) {
    if webview.label() != "main" {
        return;
    }

    let _ = webview.eval(
        r#"
        (() => {
            if (window.__externalLink) return;
            window.__externalLink = true;

            const internalOrigin = window.location.origin;
            function isExternalUrl(value) {
                try {
                    const url = new URL(value, window.location.href);
                    return url.protocol === "tel:" ||
                           (["http:", "https:"].includes(url.protocol) &&
                            url.origin !== internalOrigin);
                } catch {
                    return false;
                }
            }

            async function openExternal(link) {
                let resolved;
                try {
                    const parsed = new URL(link, window.location.href);
                    resolved = ["http:", "https:"].includes(parsed.protocol)
                        ? parsed.href
                        : parsed.href;
                } catch {
                    resolved = link;
                }

                try {
                    if (window.__TAURI__?.opener?.openUrl) {
                        await window.__TAURI__.opener.openUrl(resolved);
                        return;
                    }

                    if (window.__TAURI__?.core?.invoke) {
                        await window.__TAURI__.core.invoke("open_external", { url: resolved });
                        return;
                    }
                } catch (err) {
                    console.error("Failed to open:", resolved, err);
                }
            }

            document.addEventListener(
                "click",
                async (event) => {
                    const link = event.target?.closest?.("a[href]");
                    if (!link) return;

                    const href = link.getAttribute("href");
                    if (!href || !isExternalUrl(href)) return;

                    event.preventDefault();
                    event.stopPropagation();

                    await openExternal(href);
                },
                true
            );

            const _originalOpen = window.open;
            window.open = function (url, target, features) {
                const externalUrl = isExternalUrl(url);

                if (url && externalUrl) {
                    openExternal(url);
                    return null;
                }
                return _originalOpen.call(window, url, target, features);
            };
        })();
        "#,
    );
}

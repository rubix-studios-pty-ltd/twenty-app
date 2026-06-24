pub fn inject_link(webview: &tauri::Webview) {
    if webview.label() != "main" {
        return;
    }

    let _ = webview.eval(
        r#"
        (() => {
            if (window.__ExternalLink) return;

            window.__ExternalLink = true;

            const internalOrigin = window.location.origin;
            const originalWindowOpen = window.open.bind(window);

            function getExternalUrl(value) {
                if (!value) return null;

                try {
                    const url = new URL(value, window.location.href);

                    if (url.protocol === "mailto:") return null;
                    if (url.protocol === "tel:") return url.href;

                    if (url.protocol !== "http:" && url.protocol !== "https:") {
                        return null;
                    }

                    return url.origin === internalOrigin ? null : url.href;
                } catch {
                    return null;
                }
            }

            async function openExternal(url) {
                try {
                    await window.__TAURI__.core.invoke("open_external", { url });
                    return true;
                } catch (error) {
                    console.error("Failed to open external URL", error);
                    return false;
                }
            }

            document.addEventListener(
                "click",
                (event) => {
                    const link = event.target?.closest?.("a[href]");
                    const url = getExternalUrl(link?.getAttribute("href"));

                    if (!url) return;

                    event.preventDefault();
                    event.stopPropagation();

                    openExternal(url);
                },
                true
            );

            window.open = function (url, target, features) {
                const externalUrl = getExternalUrl(url);

                if (externalUrl) {
                    openExternal(externalUrl);
                    return null;
                }

                return originalWindowOpen(url, target, features);
            };
        })();
        "#,
    );
}

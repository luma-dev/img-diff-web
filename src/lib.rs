use leptos::*;

#[component]
pub fn SetupIonic() -> impl IntoView {
    let ionic_version = "8.2.6";
    view! {
        <script
            type="module"
            src=format!(
                "https://cdn.jsdelivr.net/npm/@ionic/core@{}/dist/ionic/ionic.esm.js",
                ionic_version,
            )

            integrity="sha384-BxNNwRAuQnCEHUMtkgl7DK31nCvc7WDLRiM3zXU7AmqZIY9ZQeagfFckfg3wNuqc"
            crossorigin="anonymous"
        ></script>
        <script
            nomodule
            src=format!(
                "https://cdn.jsdelivr.net/npm/@ionic/core@{}/dist/ionic/ionic.js",
                ionic_version,
            )

            integrity="sha384-Rq1qUz5vpQUf2P7Le7bvOSgpxjJ3Bojig83upc1RZ6aGI45tnyeuStnoa/0J/mhY"
            crossorigin="anonymous"
        ></script>
        <link
            rel="stylesheet"
            href=format!(
                "https://cdn.jsdelivr.net/npm/@ionic/core@{}/css/ionic.bundle.css",
                ionic_version,
            )

            integrity="sha384-+MFxJmfIvs5Hbt3uJs4R7/+3I/0/dcK2cfgK+P0v4RpNhnNiSRV1h3j1srM6pidg"
            crossorigin="anonymous"
        />
    }
}

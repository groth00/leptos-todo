use crate::components::sidebar::HeaderWithNavbar;
use crate::components::todo::TodoList;
use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-todo-new.css"/>

        <Title text="Todo App"/>

        <Router>
            <HeaderWithNavbar/>
            <main>
                <Routes>
                    <Route path="" view=TodoList/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="container mx-auto text-center mt-2">
            <h1 class="font-bold text-xl">"About"</h1>
            <p class="fond-semibold text-lg">"A basic app made with Leptos, a Rust framework for building CSR and SSR web applications."</p>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        use leptos::expect_context;

        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

use leptos::{
    component, create_signal, provide_context, view, IntoView, ReadSignal, SignalUpdate,
    WriteSignal,
};

const HEADER_CONTAINER_STYLE: &str =
    "bg-violet-300 p-2 mx-auto flex justify-center items-center text-center";
const ANCHOR_STYLE: &str = "block py-2 px-4 text-gray-700 hover:bg-gray-200 rounded";
const H1_STYLE: &str = "mx-auto font-bold text-xl text-center";

/// The header of the page containing the expandable sidebar on the left and
/// the centered title.
#[component]
pub fn HeaderWithNavbar() -> impl IntoView {
    let (open, set_open) = create_signal(false);

    provide_context(set_open);

    view! {
        <Header set_open/>
        <Sidebar open/>
    }
}

/// The horizontal header of the page, which contains an svg icon on the left and
/// the centered title.
/// When the svg is clicked, it will update a WriteSignal<bool> to hide/show the sidebar (part of
/// parent component).
#[component]
fn Header(set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div class=HEADER_CONTAINER_STYLE>
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="hover:text-violet-50 transition-colors duration-200"
                on:click=move |_| {
                    set_open.update(|open| *open = !*open);
                    ()
                }
            >
                <line x1="3" y1="6" x2="21" y2="6" />
                <line x1="3" y1="12" x2="21" y2="12" />
                <line x1="3" y1="18" x2="21" y2="18" />
            </svg>
            <h1 class=H1_STYLE>Todo List</h1>
        </div>
    }
}

#[component]
fn Sidebar(open: ReadSignal<bool>) -> impl IntoView {
    view! {
        <nav
            class="absolute transform transition-transform duration-200 ease-in-out"
            class=("-translate-x-full", move || !open())
        >
            <div>
                <ul class="mb-2">
                    <li class="mb-2"><a href="/" class=ANCHOR_STYLE>"Home"</a></li>
                    <li class="mb-2"><a href="/about" class=ANCHOR_STYLE>"About"</a></li>
                </ul>
            </div>
        </nav>
    }
}

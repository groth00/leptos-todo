use crate::components::types::{NotificationType, UpdateForm};
use crate::server::todo::{
    get_paginated_todos, search_todo, AddTodo, CompleteTodo, DeleteTodo, PaginatedTodos, Todo,
    UpdateTodo,
};
use leptos::html::Form;
use leptos::{
    component, create_effect, create_memo, create_node_ref, create_resource, create_rw_signal,
    create_server_action, create_signal, event_target_value, provide_context, set_timeout,
    use_context, view, Action, Callback, For, IntoView, NodeRef, ReadSignal, Resource, RwSignal,
    ServerFnError, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalWith,
    Suspense, Transition,
};
use leptos_router::ActionForm;
use leptos_use::signal_debounced;
use std::time::Duration;

const PAGE_BUTTON_STYLE: &str = "px-3 py-2 rounded-md text-sm text-gray-700 font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed border border-gray-300";
const NAV_BUTTON_STYLE: &str = "px-2 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors";
const FALLBACK_STYLE: &str = "px-2 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md opacity-50 cursor-not-allowed";

const FORM_FIELD_STYLE: &str = "pl-1 mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:border-blue-500 focus:ring focus:ring-blue-200";
const FORM_LABEL_STYLE: &str = "block text-gray-700";
const FORM_SUBMIT_STYLE: &str =
    "w-full bg-blue-400 text-white font-bold py-2 rounded-md hover:bg-blue-700";

const EDIT_FIELD_STYLE: &str = "mb-2 border-gray-300 rounded-md";

const NOTIFICATION_STYLE: &str = "hidden w-1/4 text-center fixed mx-auto top-4 inset-x-1.5 bg-green-500 text-white px-4 py-2 rounded-lg shadow-lg";

#[component]
pub fn TodoList() -> impl IntoView {
    let current_page = create_rw_signal(0u32);

    let add_action = create_server_action::<AddTodo>();
    let complete_action = create_server_action::<CompleteTodo>();
    let update_action = create_server_action::<UpdateTodo>();
    let delete_action = create_server_action::<DeleteTodo>();

    let refetch_resource = create_resource(
        move || {
            (
                add_action.version().get(),
                complete_action.version().get(),
                update_action.version().get(),
                delete_action.version().get(),
                current_page.get(),
            )
        },
        |(_, _, _, _, page)| async move { get_paginated_todos(page).await },
    );

    let form_ref = create_node_ref::<Form>();

    let (show_notification, set_show_notification) = create_signal(false);

    let (notification_type, set_notification_type) =
        create_signal::<Option<NotificationType>>(None);

    let clear_notification = move || {
        set_show_notification.update(|show| *show = false);
        set_notification_type.set(None);
    };

    create_effect(move |_| match add_action.value().get() {
        Some(Ok(_)) => {
            if let Some(form) = form_ref.get() {
                form.reset();
            }
            set_show_notification.update(|show| *show = true);
            set_notification_type.set(Some(NotificationType::SuccessAdd));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        Some(Err(e)) => {
            set_notification_type.set(Some(NotificationType::Error(e.to_string())));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        None => {}
    });

    create_effect(move |_| match update_action.value().get() {
        Some(Ok(_)) => {
            set_show_notification.update(|show| *show = true);
            set_notification_type.set(Some(NotificationType::SuccessUpdate));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        Some(Err(e)) => {
            set_notification_type.set(Some(NotificationType::Error(e.to_string())));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        None => {}
    });

    create_effect(move |_| match delete_action.value().get() {
        Some(Ok(_)) => {
            set_show_notification.update(|show| *show = true);
            set_notification_type.set(Some(NotificationType::SuccessDelete));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        Some(Err(e)) => {
            set_notification_type.set(Some(NotificationType::Error(e.to_string())));
            set_timeout(clear_notification, Duration::from_secs(1));
        }
        None => {}
    });

    provide_context(current_page);
    provide_context(refetch_resource);

    provide_context(add_action);
    provide_context(complete_action);
    provide_context(update_action);
    provide_context(delete_action);

    provide_context(show_notification);
    provide_context(notification_type);
    provide_context(form_ref);

    let todos = move || {
        refetch_resource().map(|result| match result {
            Ok(todos) => {
                if todos.items.is_empty() {
                    view! { <p>"You finished all of your todo items!"</p> }.into_view()
                } else {
                    view! {
                        {
                            todos.items.into_iter().map(|todo| {
                                view! { <TodoItem todo/> }
                            })
                            .collect::<Vec<_>>()
                        }
                    }
                    .into_view()
                }
            }
            Err(e) => view! {
                <p>"Error loading todos: "{e.to_string()}</p>
            }
            .into_view(),
        })
    };

    view! {
        <NotificationComponent/>
        <div class="container mx-auto flex mt-6">
            <FormAddTodo/>

            <div class="w-3/4">
                <div class="space-y-4">
                    <Search/>
                    <Transition fallback=move || view! { <p>"Loading todos..."</p> }>
                        {todos}
                    </Transition>
                    <Pagination/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PageButton(
    #[prop(into)] page: u32,
    #[prop(into)] current_page: u32,
    #[prop(into)] on_click: Callback<u32>,
) -> impl IntoView {
    view! {
        <button
            class=PAGE_BUTTON_STYLE
            class=("bg-blue-100", page == current_page)
            on:click=move |_| on_click(page)
        >
            {page + 1}
        </button>
    }
}

#[component]
fn PaginationControls(
    current_page: RwSignal<u32>,
    total_pages: u32,
    visible_pages: Vec<u32>,
) -> impl IntoView {
    let visible_pages = create_memo(move |_| visible_pages.clone());

    let go_to_page = move |page: u32| current_page.set(page);

    let go_to_prev = move |_| {
        if current_page.get() > 0 {
            current_page.update(|p| *p -= 1);
        }
    };

    let go_to_next = move |_| {
        if current_page.get() + 1 < total_pages {
            current_page.update(|p| *p += 1);
        }
    };

    let go_to_first = move |_| {
        current_page.update(|p| *p = 0);
    };

    let go_to_last = move |_| {
        current_page.update(|p| *p = total_pages - 1);
    };

    let show_start_ellipsis =
        move || visible_pages.with(|pages| pages.first().copied().unwrap_or(0) > 0);

    let show_end_ellipsis =
        move || visible_pages.with(|pages| pages.last().copied().unwrap_or(0) < total_pages - 1);

    view! {
        <div class="flex items-center justify-center space-x-2 my-4">
            <button class=NAV_BUTTON_STYLE on:click=go_to_first disabled=move || current_page() == 0>"First"</button>
            <button class=NAV_BUTTON_STYLE on:click=go_to_prev disabled=move || current_page() == 0>"Previous"</button>

            {move || {
                if show_start_ellipsis() {
                    view! { <span class="px-2 py-2 text-gray-500">"..."</span> }
                } else {
                    view! { <span></span> }
                }
            }}

            <For
                each=move || visible_pages()
                key=|page| *page
                children=move |page| {
                    view! { <PageButton page=page current_page=current_page() on_click=go_to_page/> }
                }
            />

            {move || {
                if show_end_ellipsis() {
                    view! { <span class="px-2 py-2 text-gray-500">"..."</span> }
                } else {
                    view! { <span></span> }
                }
            }}

            <button class=NAV_BUTTON_STYLE on:click=go_to_next disabled=move || { current_page() + 1 >= total_pages }>"Next"</button>
            <button class=NAV_BUTTON_STYLE on:click=go_to_last disabled=move || { current_page() + 1 >= total_pages }>"Last"</button>
        </div>
    }
}

#[component]
fn PaginationFallback() -> impl IntoView {
    view! {
        <div class="space-y-4 opacity-60">
            <div class="text-center py-4">"Loading..."</div>
            <div class="flex items-center justify-center space-x-2">
                <button class=FALLBACK_STYLE disabled>"«"</button>
                <button class=FALLBACK_STYLE disabled>"1"</button>
                <button class=FALLBACK_STYLE disabled>"»"</button>
            </div>
        </div>
    }
}

#[component]
fn Pagination() -> impl IntoView {
    const PER_PAGE: u32 = 10;
    const VISIBLE_PAGES: u32 = 5;

    let current_page =
        use_context::<RwSignal<u32>>().expect("need current_page RwSignal for pagination");

    let todos = use_context::<
        Resource<(usize, usize, usize, usize, u32), Result<PaginatedTodos, ServerFnError>>,
    >()
    .expect("need refetch_resource for pagination");

    view! {
        <div class="w-full max-w-4xl mx-auto">
            <Transition fallback=move || view! { <PaginationFallback/> }>
                {move || {
                    todos.get().map(|data| {
                        match data {
                            Ok(response) => {
                                let total_pages = (response.total + PER_PAGE - 1) / PER_PAGE;

                                // Calculate visible pages
                                let half_visible = VISIBLE_PAGES / 2;
                                let current = current_page.get();
                                let start_page = if current > half_visible {
                                    if current + half_visible >= total_pages {
                                        total_pages.saturating_sub(VISIBLE_PAGES)
                                    } else {
                                        current.saturating_sub(half_visible)
                                    }
                                } else {
                                    0
                                };

                                let end_page = (start_page + VISIBLE_PAGES).min(total_pages);
                                let visible_pages = (start_page..end_page).collect::<Vec<_>>();

                                view! {
                                    <PaginationControls
                                        current_page=current_page
                                        total_pages=total_pages
                                        visible_pages=visible_pages
                                    />
                                }.into_view()
                            }
                            Err(_) => view! { <div>"Error loading pagination"</div> }.into_view()
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
fn NotificationComponent() -> impl IntoView {
    let show_notification = use_context::<ReadSignal<bool>>()
        .expect("need show_notification ReadSignal to show/hide notification");

    let notification_type = use_context::<ReadSignal<Option<NotificationType>>>()
        .expect("need notification type to display message");

    let notification_message = move || match notification_type().take() {
        Some(NotificationType::SuccessAdd) => "Todo item added successfully!".to_string(),
        Some(NotificationType::SuccessUpdate) => "Todo item updated successfully!".to_string(),
        Some(NotificationType::SuccessDelete) => "Todo item deleted successfully!".to_string(),
        Some(NotificationType::Error(e)) => e,
        None => "".to_string(),
    };

    view! {
        <div class=NOTIFICATION_STYLE class:hidden=move || { !show_notification() }>
            {notification_message}
        </div>
    }
}

#[component]
fn FormAddTodo() -> impl IntoView {
    let add_action = use_context::<Action<AddTodo, Result<(), ServerFnError>>>()
        .expect("need action for adding a todo item");

    let form_ref =
        use_context::<NodeRef<Form>>().expect("need NodeRef<Form> to reset the form on submission");

    let today = chrono::offset::Local::now().date_naive().to_string();

    view! {
        <div class="w-1/4 bg-white p-4 rounded-lg shadow-md mr-6">
            <h2 class="text-lg font-bold mb-4">Add New To-Do</h2>
            <ActionForm action=add_action node_ref=form_ref>
                <div class="mb-4">
                    <label for="title" class=FORM_LABEL_STYLE>Title</label>
                    <input name="title" type="text" placeholder="Enter title" required class=FORM_FIELD_STYLE/>
                </div>
                <div class="mb-4">
                    <label for="description" class=FORM_LABEL_STYLE>Description</label>
                    <textarea name="description" rows="3" placeholder="Enter description" class=FORM_FIELD_STYLE></textarea>
                </div>
                <div class="mb-4">
                    <label for="due_date" class=FORM_LABEL_STYLE>Due Date</label>
                    <input name="due_date" type="date" class=FORM_FIELD_STYLE value={today} required/>
                </div>
                <button
                    type="submit"
                    class=FORM_SUBMIT_STYLE
                    prop:disabled=move || add_action.pending().get()
                >
                    {move || {
                        if add_action.pending().get() {
                            "Adding..."
                        } else {
                            "Add Todo"
                        }
                    }}
                </button>
            </ActionForm>
        </div>
    }
}

#[component]
fn Search() -> impl IntoView {
    let complete_action = use_context::<Action<CompleteTodo, Result<(), ServerFnError>>>()
        .expect("need complete_action to update search results");

    let (query, set_query) = create_signal(String::new());
    let debounced: Signal<String> = signal_debounced(query, 500.0);

    let todos = create_resource(
        move || (debounced(), complete_action.version().get()),
        |(q, _)| async move { search_todo(q).await },
    );

    let todos_result = move || match todos() {
        None => view! {}.into_view(), // unreachable
        Some(Ok(todos)) => {
            if todos.is_empty() {
                view! {}.into_view()
            } else {
                view! {
                    <div class="container mx-auto mb-4 border-b-2 border-blue-500">
                        <For
                            each=move || todos.clone()
                            key=|todo| todo.id
                            children=move |todo| view! {
                                <TodoItem todo/>
                            }
                        />
                    </div>
                }
                .into_view()
            }
        }
        Some(Err(e)) => view! {
            <p>"Search error: "{e.to_string()}</p>
        }
        .into_view(),
    };

    view! {
        <span>
            <input
                type="text"
                name="search"
                on:input=move |ev| {
                    set_query.set(event_target_value(&ev));
                }
                prop:value=query
                class="placeholder:italic placeholder:text-slate-400 block bg-white w-md mx-auto
                    border border-slate-300 rounded-md py-2 pl-3 pr-3 
                    shadow-sm focus:outline-none focus:border-sky-500 focus:ring-sky-500 focus:ring-1 sm:text-sm" 
                placeholder="Search"
            />
            <Suspense fallback=move || view! {}>
                {todos_result}
            </Suspense>
        </span>
    }
}

#[component]
fn TodoItem(todo: Todo) -> impl IntoView {
    let complete_action = use_context::<Action<CompleteTodo, Result<(), ServerFnError>>>()
        .expect("need complete_action to trigger server function");

    let delete_action = use_context::<Action<DeleteTodo, Result<(), ServerFnError>>>()
        .expect("need delete_action to trigger server function");

    let hidden = create_rw_signal(true);

    provide_context(hidden);

    let on_complete = move |_| complete_action.dispatch(CompleteTodo { id: todo.id });
    let on_delete = move |_| delete_action.dispatch(DeleteTodo { id: todo.id });
    let on_edit = move |_| {
        hidden.update(|hidden| *hidden = false);
    };

    view! {
        <div class="flex items-start border-b border-gray-300 pb-4 mb-4">
            <input
                type="checkbox"
                checked=todo.completed
                on:change=on_complete
                class="mr-4 h-5 w-5 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                class:disabled=move || !hidden()
            />

            <div class="flex-grow">
                // content
                <div class="view" class:hidden=move || !hidden() on:click=on_edit>
                    <h3 class="text-lg font-semibold text-gray-800">{todo.title.clone()}</h3>
                    <p class="text-gray-600 selection:text-sky-500">{todo.description.clone()}</p>
                    <p class="text-sm text-gray-500 mt-1">Due Date: <span class="font-medium">{todo.due_date.clone()}</span></p>
                </div>

                <FormUpdateTodo todo/>

            </div>

            // delete button
            <div class="flex space-x-2 ml-4">
                <button class="text-red-600 hover:text-red-800" on:click=on_delete>
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}

/// Allow a user to edit a todo-item inline.
/// When the user clicks on the div containing the title, description, and due_date,
/// the original content is hidden and corresponding inputs and a save button are revealed.
/// When the user submits the form by clicking on the save button,
/// it triggers a server function to update the row in the database.
/// Then the inputs and save button are hidden and the item is re-rendered.
#[component]
fn FormUpdateTodo(todo: Todo) -> impl IntoView {
    let form_state = create_rw_signal(UpdateForm {
        title: todo.title,
        description: todo.description,
        due_date: todo.due_date,
    });

    let hidden = use_context::<RwSignal<bool>>().expect("need hidden to show edit inputs");

    let update_action = use_context::<Action<UpdateTodo, Result<(), ServerFnError>>>()
        .expect("need update_action to call server function");

    let on_submit = move |_| {
        update_action.dispatch(UpdateTodo {
            id: todo.id,
            title: form_state().title,
            description: form_state().description,
            due_date: form_state().due_date,
        });
        hidden.update(|hidden| *hidden = true);
    };

    view! {
        <div class="flex flex-col" class:hidden=move || hidden()>
            <input
                type="text"
                required
                class=EDIT_FIELD_STYLE
                value=move || form_state().title
                on:input=move |ev| {
                    form_state.update(|state| {
                        if event_target_value(&ev) != "" {
                            state.title = event_target_value(&ev)
                        }
                    })
                }
            />
            <textarea
                rows=3
                class=EDIT_FIELD_STYLE
                on:input=move |ev| {
                    form_state.update(|state| state.description = event_target_value(&ev))
                }
                prop:value=move || form_state().description
            >
                {form_state.get_untracked().description}
            </textarea>
            <input
                type="date"
                required
                class=EDIT_FIELD_STYLE
                value=move || form_state().due_date
                on:input=move |ev| {
                    form_state.update(|state| {
                        if event_target_value(&ev) != "" {
                            state.due_date = event_target_value(&ev)
                        }
                    })
                }
            />
        </div>

        <button class="text-green-600 hover:text-green-800" class:hidden=move|| hidden() on:click=on_submit>
            <span class="flex items-center">
                <p>Save</p>
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12l5 5L20 7"/>
                </svg>
            </span>
        </button>
    }
}

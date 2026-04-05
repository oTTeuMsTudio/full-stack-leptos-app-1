use leptos::prelude::*;

#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    // Main todos signal
    let (todos, set_todos) = create_signal::<Vec<Todo>>(vec![]);

    // New task input
    let (new_task, set_new_task) = create_signal(String::new());

    // Filter state: "all" | "active" | "completed"
    let (filter, set_filter) = create_signal("all".to_string());

    // Add a new todo
    let add_todo = move |_| {
        let task = new_task.get().trim().to_string();
        if !task.is_empty() {
            set_todos.update(|ts| {
                let new_id = ts.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                ts.push(Todo {
                    id: new_id,
                    task,
                    completed: false,
                });
            });
            set_new_task.set(String::new()); // Clear input
        }
    };

    // Toggle completion
    let toggle_todo = move |id: usize| {
        set_todos.update(|ts| {
            if let Some(todo) = ts.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    // Delete a todo
    let delete_todo = move |id: usize| {
        set_todos.update(|ts| {
            ts.retain(|t| t.id != id);
        });
    };

    // Filtered todos (derived)
    let filtered_todos = move || {
        let current_filter = filter.get();
        todos.get()
            .into_iter()
            .filter(|todo| match current_filter.as_str() {
                "active" => !todo.completed,
                "completed" => todo.completed,
                _ => true, // "all"
            })
            .collect::<Vec<_>>()
    };

    // Count helpers
    let total_todos = move || todos.get().len();
    let completed_count = move || todos.get().iter().filter(|t| t.completed).count();

    view! {
        <div class="min-h-screen bg-zinc-950 py-12 px-6">
            <div class="max-w-xl mx-auto">
                <!-- Header -->
                <div class="text-center mb-12">
                    <h1 class="text-6xl font-bold tracking-tighter mb-3 text-white">"todos"</h1>
                    <p class="text-zinc-400">"A classic Leptos example"</p>
                </div>

                <!-- Add new todo form -->
                <div class="bg-zinc-900 rounded-3xl p-6 mb-8 border border-zinc-800">
                    <form 
                        on:submit=move |ev| {
                            ev.prevent_default();
                            add_todo(());
                        }
                        class="flex gap-3"
                    >
                        <input
                            type="text"
                            placeholder="What needs to be done?"
                            class="flex-1 bg-zinc-800 border border-zinc-700 rounded-2xl px-6 py-4 text-lg focus:outline-none focus:border-blue-500 transition-colors placeholder:text-zinc-500"
                            prop:value=new_task
                            on:input=move |ev| {
                                set_new_task.set(event_target_value(&ev));
                            }
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    add_todo(());
                                }
                            }
                        />
                        <button
                            type="submit"
                            class="px-8 bg-blue-600 hover:bg-blue-700 rounded-2xl font-semibold transition-all active:scale-95"
                        >
                            "Add"
                        </button>
                    </form>
                </div>

                <!-- Todo List -->
                <div class="bg-zinc-900 rounded-3xl overflow-hidden border border-zinc-800 mb-8">
                    <Show
                        when=move || !filtered_todos().is_empty()
                        fallback=|| view! {
                            <div class="p-12 text-center text-zinc-500">
                                "No todos yet. Add one above! ✨"
                            </div>
                        }
                    >
                        <div class="max-h-96 overflow-y-auto">
                            <For
                                each=filtered_todos
                                key=|todo| todo.id
                                let:todo
                            >
                                <div class="flex items-center gap-4 px-6 py-5 border-b border-zinc-800 last:border-none hover:bg-zinc-800/50 transition-colors group">
                                    <input
                                        type="checkbox"
                                        checked=todo.completed
                                        on:change=move |_| toggle_todo(todo.id)
                                        class="w-5 h-5 accent-blue-500 cursor-pointer"
                                    />
                                    <span 
                                        class=move || format!(
                                            "flex-1 text-lg {}",
                                            if todo.completed { "line-through text-zinc-500" } else { "text-white" }
                                        )
                                    >
                                        {todo.task.clone()}
                                    </span>
                                    <button
                                        on:click=move |_| delete_todo(todo.id)
                                        class="opacity-0 group-hover:opacity-100 text-red-500 hover:text-red-600 transition-all px-3 py-1"
                                    >
                                        "✕"
                                    </button>
                                </div>
                            </For>
                        </div>
                    </Show>
                </div>

                <!-- Footer stats & filters -->
                <div class="flex flex-wrap items-center justify-between text-sm text-zinc-400 px-2">
                    <div>
                        {move || format!("{} items left", total_todos() - completed_count())}
                    </div>

                    <!-- Filter buttons -->
                    <div class="flex gap-2">
                        <button
                            on:click=move |_| set_filter.set("all".to_string())
                            class=move || format!("px-4 py-1 rounded-xl transition-colors {}", 
                                if filter.get() == "all" { "bg-zinc-800 text-white" } else { "hover:bg-zinc-900" })
                        >
                            "All"
                        </button>
                        <button
                            on:click=move |_| set_filter.set("active".to_string())
                            class=move || format!("px-4 py-1 rounded-xl transition-colors {}", 
                                if filter.get() == "active" { "bg-zinc-800 text-white" } else { "hover:bg-zinc-900" })
                        >
                            "Active"
                        </button>
                        <button
                            on:click=move |_| set_filter.set("completed".to_string())
                            class=move || format!("px-4 py-1 rounded-xl transition-colors {}", 
                                if filter.get() == "completed" { "bg-zinc-800 text-white" } else { "hover:bg-zinc-900" })
                        >
                            "Completed"
                        </button>
                    </div>

                    <button
                        on:click=move |_| set_todos.set(vec![])
                        class="text-red-500 hover:text-red-400 transition-colors"
                    >
                        "Clear all"
                    </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> })
}

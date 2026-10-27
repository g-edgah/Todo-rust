use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;


fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {

    let (title, title_writer) = signal(String::new());
    let (items, items_writer) = signal(Vec::<String>::new());

    view! {
        <main> 
            <div>
                <div class="list">
                    list of items

                    <For
                        each = move || items.get()
                        key = |item|  item.clone()
                        children = move |task| { 
                            let task_to_go = task.clone();

                            view! { 
                                <div>{task.clone()}</div> <button
                                    type="button"
                                    on:click=move |_| {
                                        items_writer.update(|items| {
                                            items.retain(|item| item != &task_to_go);
                                        });

                                    }
                                >
                                    Remove
                                </button>
                            }
                        }
                    />
                </div>

                <form 
                    action="" 
                    on:submit=move|ev: SubmitEvent| {
                        ev.prevent_default();
                        items_writer.write().push(title.get());
                        title_writer.set(String::new());

                    }
                >
                    
                    <input 
                        type="text" 
                        on:input=move|event| {
                        title_writer.set(event_target_value(&event));
                        } 
                        prop:value = move || title.get()
                    />

                    <button id="submit" type="submit">
                        Add item
                    </button>

                </form>

            </div>
        </main> 
    }
}

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
                        each = move ||items.get()
                        key = |item|  item.clone()
                        children = move |task| { 
                            view! { 
                                <div>{task}</div>
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

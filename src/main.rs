use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;


fn main() {
    mount_to_body(App);
}


#[derive(Clone, Default)]
struct Item { 
    id : u32,
    title : String,
    description : String
}


#[component]
fn App() -> impl IntoView {

    let (title, title_writer) = signal(Item::default());
    let (items, items_writer) = signal(Vec::<Item>::new());
    let (counter, counter_writer) = signal(0);


    view! {
        <main> 
            <div>
                <div class="list">
                    list of items

                    <For
                        each = move || items.get()
                        key = |item| item.id
                        children = move |item| { 
                            let task_to_go = item.id;

                            view! { 
                                <div>
                                    <span>{item.title}</span>
                                    <span>{item.description}</span>
                                </div> 
                                <button
                                    type="button"
                                    on:click= { move |_| {
                                        items_writer.update(|tasks| {
                                            tasks.retain(|task| task.id != task_to_go);
                                        });

                                    }}
                                >
                                    Remove
                                </button>
                            }
                        }
                    />
                </div>

                <form 
                    on:submit = { move|ev: SubmitEvent| {
                        ev.prevent_default();

                        items_writer.write().push(Item {
                            id : counter.get(),
                            title : title.get().title,
                            description: title.get().description
                        });

                        title_writer.set(Item::default());

                        counter_writer.set(counter.get() +1);

                    }}
                >
                    
                    <input 
                        type = "text" 
                        on:input = { move|event| {
                        title_writer.write().title = event_target_value(&event);
                        } }
                        prop:value = move || title.get().title
                    />

                    <button id="submit" type="submit">
                        Add item
                    </button>

                </form>

            </div>
        </main> 
    }
}

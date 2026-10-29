use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;


fn main() {
    mount_to_body(App);
}


#[derive(Clone)]
struct Item { 
    id : u32,
    title : String
}


#[component]
fn App() -> impl IntoView {

    let (title, title_writer) = signal(String::new());
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
                            //let task_to_go = item.title;

                            view! { 
                                <div>{item.title}</div> 
                                <button
                                    // type="button"
                                    // on:click=move |_| {
                                    //     items_writer.update(|items| {
                                    //         items.retain(|item| item.title != &task_to_go);
                                    //     });

                                    // }
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
                        counter_writer.set(counter.get() +1);

                        items_writer.write().push(Item {
                            id : counter.get(),
                            title : title.get()
                        });
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

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
        <main class="flex flex-col"> 
            <div class="flex flex-col justify-center gap-5 w-full">
                <div class="list bg-gray-500 min-h-100 w-full p-10 gap-5">
                    <div class="text-xl font-bold mb-5" >list of items</div>

                    <div class="flex flex-col gap-2 w-full">

                        <For
                            each = move || items.get()
                            key = |item| item.id
                            children = move |item| { 
                                let task_to_go = item.id;

                                view! { 
                                    <div class="flex gap-5 min-h-10 items-center min-w-0 w-9/10 justify-between text-wrap">
                                        <div class="flex flex-col w-full text-wrap">
                                            <span class="min-w-0">{item.title}</span>
                                            <span class="min-w-0">{item.description}</span>
                                        </div> 
                                        <button
                                            class="text-red-800 w-30 h-full rounded-sm"
                                            type="button"
                                            on:click= { move |_| {
                                                items_writer.update(|tasks| {
                                                    tasks.retain(|task| task.id != task_to_go);
                                                });

                                            }}
                                        >
                                            Remove
                                        </button>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                <div class="w-full flex bg-gray-400 min-h-60 p-5 justify-center">

                    <form 
                        class="flex gap-3 h-11"
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
                            class="w-100 bg-white rounded-md px-2"
                            type = "text" 
                            on:input = { move|event| {
                            title_writer.write().title = event_target_value(&event);
                            } }
                            prop:value = move || title.get().title
                        />

                        <button class="bg-gray-600 w-30 h-full rounded-sm" id="submit" type="submit">
                            Add item
                        </button>

                    </form>
                </div>

            </div>
        </main> 
    }
}

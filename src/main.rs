use leptos::prelude::*;


fn main() {
    mount_to_body(App);
}


#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <div>
                <div class="list">
                    list of items
                </div>
                <form action="">
                    <input type="text" />
                    <button id="submit">Add item</button>
                </form>
            </div>
        </main> 
    }
}

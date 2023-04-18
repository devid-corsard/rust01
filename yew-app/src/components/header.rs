use yew::prelude::*;

#[function_component]
pub fn Header() -> Html{
    html! {
    <nav class="navbar bg-black">
        <div class="container-fluid">
            <a class="navbar-brand text-white" href="#">{"User list"}</a>
        </div>
    </nav>
    }
}

use yew::prelude::*;

#[function_component]
pub fn Loader() -> Html{
    html! {
    <div class="spinner-border" role="status">
        <span class="visually-hidden">{"Loading..."}</span>
    </div>
    }

}

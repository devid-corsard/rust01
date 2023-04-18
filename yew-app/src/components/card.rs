use yew::prelude::*;
use crate::models::user::User;

#[derive(Properties, PartialEq)]
pub struct CardProp {
    pub user: User,
}

#[function_component]
pub fn Card(CardProp { user }: &CardProp) -> Html {
    html! {
    <div class="m-3 p-4 border rounded d-flex align-items-center">
        <img src={format! ("{}", &user.picture.large)} class="img-thumbnail mr-2 w-25 p-3" alt="img" />
        <div class="m-4">
            <p class="fw-bold mb-1">{format!("{} {} {}", &user.name.title, &user.name.first, &user.name.last)}</p>
            <p class="fw-normal mb-1">{format!("{}, {}", &user.gender, &user.dob.age)}</p>
            <p class="fw-normal mb-1">{&user.email}</p>
            <p class="fw-normal mb-1">{&user.phone}</p>
            <p class="fw-normal mb-1">{format!("{}, {}", &user.location.country, &user.location.city)}</p>
        </div>
    </div>
    }
}

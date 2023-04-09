use crate::pages::{create::Create, home::Home, question::Question};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/question")]
    Create,
    #[at("/question/:id")]
    Question { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Create => html! { <Create /> },
        Route::Home => html! { <Home /> },
        Route::Question { id } => html! { <Question id={id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

mod pages;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::Scope;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/skills")]
    Skills,
    #[at("/cv")]
    CV,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
}

pub enum Msg {
    ToggleNavbar,
}
pub struct Model {
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }
                <main style="margin-top: 81px; width: 100vw">
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-fixed-top"
                style="background-color: #000005; border-width: 0px 0px 2px 0px;"
                role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-5">{ "Gheo Tech" }</h1>
                    <button class={classes!("navbar-burger", "burger", "button", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}
                        onclick={link.callback(|_| Msg::ToggleNavbar)}>
                    <div class="navbar-end">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Skills}>
                            { "skills" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::CV}>
                            { "cv" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Projects}>
                            { "projects" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                            { "contact" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => html! { <pages::home::Home/> },
        Route::Skills => html! { <pages::skills::Skills/> },
        Route::CV => html! { <pages::cv::CV/> },
        Route::Projects => html! { <pages::projects::Projects /> },
        Route::Contact => html! { <pages::contact::Contact /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}

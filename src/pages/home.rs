use yew::prelude::*;
use crate::{Route, Link};

static DEMO_CODE: &'static str = include_str!("../components/data/home_demo.rs");

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="container fade-in" style="margin-top: -27px">
            <div class="columns is-vcentered is-gapless">
                <div class="column is-7 has-text-centered homecolumnleft">
                <div class="container" style="max-width: 450px">
                  <br/>
                  <div class="title is-4">
                      {"Gheorghe Technologies SRL"}
                  </div>
                  <div class="subtitle is-5 is-italic">
                      {"Software Development, DevOps, Web2.0+Web3.0"}
                  </div>
                  <p>{"Welcome to Gheorghe's portfolio website!"}</p>
                  <br/>
                  <p>{"This website should offer you all the information needed in case
                      you are seeking development or consultancy services, including:
                      Programming (Rust, GoLang), Linux, Cloud, DevOps,
                      Infrastructure Architecture,
                      Blockchain, dApps."}</p>
                  <div class="container" style="max-width: 250px">
                      <br/>
                      <Link<Route> to={Route::Skills}>
                          <button class="button is-white is-outlined is-fullwidth">
                             {"My Skills"}
                          </button>
                      </Link<Route>>
                      <br/>
                      <Link<Route> to={Route::CV}>
                          <button class="button is-white is-outlined is-fullwidth">
                             {"Curriculum Vitae"}
                          </button>
                      </Link<Route>>
                      <br/>
                      <Link<Route> to={Route::Projects}>
                          <button class="button is-white is-outlined is-fullwidth">
                              {"Open-Source Projects"}
                          </button>
                      </Link<Route>>
                      <br/>
                      <Link<Route> to={Route::Contact}>
                          <button class="button is-white is-outlined is-fullwidth">
                              {"Contact Information"}
                          </button>
                      </Link<Route>>
                      <br/>
                      <a href="https://blog.gheo.tech">
                          <button class="button is-black is-fullwidth">
                              {"Blog"}
                          </button>
                      </a>
                      <br/>
                  </div>
                </div>
                </div>
                <div class="column homecolumnright">
                    <pre class="homedemocode">{DEMO_CODE}</pre>
                </div>
            </div>
        </div>
        }
    }
}

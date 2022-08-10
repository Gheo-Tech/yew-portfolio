use yew::prelude::*;
use crate::components::projects::{Project, ProjectCard};

pub struct Projects;
impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 1000px">
               <div class="subtitle is-4 has-text-centered">{"Projects"}</div>
               < ProjectCard project={Project::GheoTech} />
               < ProjectCard project={Project::MerkleTree} />
               < ProjectCard project={Project::RustPhantom} />
               < ProjectCard project={Project::ActixServer} />
            </div>
        }
    }
}

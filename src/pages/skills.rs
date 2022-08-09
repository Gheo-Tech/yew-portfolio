use yew::prelude::*;
use crate::components::skills::SkillList;

pub struct Skills;
impl Component for Skills {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in">
                <div class="subtitle is-4 has-text-centered">
                    {"Skills"}
                </div>
                <SkillList/>
            </div>
        }
    }
}

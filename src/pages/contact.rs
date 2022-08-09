use yew::prelude::*;

pub struct Contact;
impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 600px; height: 100vh">
            <div class="columns is-vcentered has-text-centered" style="min-height: 80vh"><div class="column">
                <div class="subtitle is-4">
                    {"Contact Information"}
                </div>

                <div class="subtitle is-7 has-text-centered">
                    {"Due to the advanced bot protection, copy-paste will not work."}
                </div>
                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right"
                        style="margin-top: 10px">{"email:"}</div>
                    <div class="column">
                        <img style="max-height: 42px"
                            src="https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/email.png"/>
                    </div>
                </div>
                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right"
                        style="margin-top: 10px">{"phone:"}</div>
                    <div class="column">
                        <img style="max-height: 42px;"
                            src="https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/number.png"/>
                    </div>
                </div>
                <div class="subtitle is-7 has-text-centered">
                    {"Signal, WhatsApp and Telegram are available for this number."}
                </div>

                <br/><br/><br/>
            </div></div></div>
        }
    }
}

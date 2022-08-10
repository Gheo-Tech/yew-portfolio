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
            <div class="container fade-in" style="max-width: 600px; height: 90vh">
            <div class="columns is-vcentered" style="min-height: 80vh"><div class="column">

                <div class="columns is-mobile">
                    <div class="column is-4"></div>
                    <div class="column">
                        <div class="subtitle is-4">
                            {"Contact Information"}
                        </div>
                    </div>
                </div>


                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right">{"schedule:"}</div>
                    <div class="column">{"11:00 am - 01:00 am UTC"}
                        <div class="subtitle is-7" style="margin-top: 10px">
                            {"Available every day of the week during these hours."}
                        </div>
                    </div>
                </div>
                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right">{"email:"}</div>
                    <div class="column">
                        <img style="max-height: 42px; margin-left:-7px; margin-top: -10px"
                            src="https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/email.png"/>
                        <div class="subtitle is-7">
                            {"Due to the advanced bot protection, copy-paste will not work."}
                        </div>
                    </div>
                </div>
                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right">{"phone:"}</div>
                    <div class="column">
                        <img style="max-height: 42px; margin-left:-7px; margin-top: -10px"
                            src="https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/number.png"/>
                        <div class="subtitle is-7">
                            {"Signal, WhatsApp and Telegram are available for this number."}
                        </div>
                    </div>
                </div>

                <div class="columns is-mobile">
                    <div class="column is-4 has-text-right"></div>
                    <div class="column" style="max-width: 400px">
                        <div class="jobtext">
                            {"...or just go for "}
                            <a target="_blank" href="https://www.linkedin.com/in/gun1x/">
                                {"linkedin"}
                            </a>
                        </div>
                    </div>
                </div>

                <br/><br/><br/>
            </div></div></div>
        }
    }
}

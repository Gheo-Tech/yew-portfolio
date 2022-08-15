use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Job {
    GheoTech,
    Kape,
    Vodafone,
    IBM,
    Freelance,
}

struct JobMeta {
    title: String,
    company: String,
    start: String,
    end: String,
    story: String,
    tech: String,
    resp: String,
}

impl From<&Job> for JobMeta {
    fn from(j: &Job) -> Self {
        macro_rules! match_j {
            ( $( $job_key:ident => {
                    $title:expr, $company:expr, $start: expr, $end: expr, $file:expr
                }, )+
            ) => { match j {
                $( Job::$job_key => JobMeta {
                    title: $title.to_string(),
                    company: $company.to_string(),
                    start: $start.to_string(),
                    end: $end.to_string(),
                    story: include_str!(concat!("data/", $file, "_story.html")).to_string(),
                    tech: include_str!(concat!("data/", $file, "_tech.html")).to_string(),
                    resp: include_str!(concat!("data/", $file, "_resp.html")).to_string(),
                }, )+
            }  };
        }
        match_j!(
            GheoTech => { "Engineer, Consultant", "Gheorghe Technologies SRL", "08/2022", "present", "gheotech" },
            Kape => { "Infrastructure Architect", "Kape", "09/2018", "07/2022", "kape" },
            Vodafone => { "Private Cloud Engineer", "Vodafone", "04/2015", "09/2018", "vodafone" },
            IBM => { "Java Dev, Sysadmin", "IBM", "05/2014", "04/2015", "ibm" },
            Freelance => { "Freelancer", "Multiple Projects", "2010", "2015", "freelance" },
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TextVisibility {
    None,
    Story,
    Tech,
    Resp,
}

impl TextVisibility {
    fn to_string(&self) -> String {
        match self {
            TextVisibility::None => String::new(),
            TextVisibility::Story => "story".to_string(),
            TextVisibility::Tech => "tech".to_string(),
            TextVisibility::Resp => "scope".to_string(),
        }
    }
}

pub enum Msg {
    Buttons,
    Text(TextVisibility),
}

impl TextVisibility {
    fn show_story(&self) -> bool {
        match self {
            TextVisibility::Story => true,
            _ => false,
        }
    }
    fn show_tech(&self) -> bool {
        match self {
            TextVisibility::Tech => true,
            _ => false,
        }
    }
    fn show_resp(&self) -> bool {
        match self {
            TextVisibility::Resp => true,
            _ => false,
        }
    }
}

pub struct JobCard {
    job: JobMeta,
    text: TextVisibility,
    menu: bool,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub job: Job,
}

impl Component for JobCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            job: JobMeta::from(&ctx.props().job),
            text: TextVisibility::None,
            menu: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Text(text_visibility) => {
                self.text = match text_visibility == self.text {
                    true => TextVisibility::None,
                    false => match self.text {
                        TextVisibility::None => text_visibility,
                        _ => {
                            let link = ctx.link().clone();
                            spawn_local(async move {
                                TimeoutFuture::new(1000).await;
                                link.send_message(Msg::Text(text_visibility.clone()));
                            });
                            TextVisibility::None
                        }
                    },
                };
            }
            Msg::Buttons => self.menu = !self.menu,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let story_div = gloo_utils::document().create_element("div").unwrap();
        story_div.set_inner_html(&self.job.story);
        let tech_div = gloo_utils::document().create_element("div").unwrap();
        tech_div.set_inner_html(&self.job.tech);
        let resp_div = gloo_utils::document().create_element("div").unwrap();
        resp_div.set_inner_html(&self.job.resp);

        let menu_class = match self.menu {
            true => "jobmenu-visible",
            false => "jobmenu-hidden",
        };

        let menu_button_weight = |v: TextVisibility| match self.text == v {
            true => "jobbuttonbolder",
            false => "jobbutton",
        };

        fn text_class(active: bool) -> &'static str {
            match active {
                true => "jobtext-visible",
                false => "jobtext-hidden",
            }
        }

        macro_rules! show_button {
            ($title:expr) => {{
                html! {
                    <div class="column is-4 has-text-centered">
                        <button style="max-width: 100px; height: 28px;"
                            class={classes!("button", "is-white", "is-outlined",
                            menu_button_weight(TextVisibility::$title))}
                            onclick={ctx.link().callback(|_| Msg::Text(TextVisibility::$title))}>
                        {TextVisibility::$title.to_string()}</button>
                    </div>
                }
            }};
        }

        macro_rules! show_text {
            ($title:expr, $func:expr, $div:expr) => {{
                html! {
                    <div class={classes!("jobtext", text_class(self.text.$func && self.menu))}>
                        {Html::VRef($div.into())}
                    </div>
                }
            }};
        }

        html! {
            <div class="box jobbox">
                <div class="columns is-mobile is-centered">
                    <div class="column is-5">
                        <div class="title is-5">{&self.job.title}</div>
                        <div class="subtitle is-6">{&self.job.company}</div>
                    </div>
                    <div class="column is-2 has-text-centered">
                        <div class="button is-white is-outlined jobbutton"
                            style="height: 26px; width: 28px; padding: 4px 0px 0px 0px; border: 0px;"
                            onclick={ctx.link().callback(|_| Msg::Buttons)}>
                            if !self.menu { { "▼" } } else { { "▲" } }
                        </div>
                    </div>
                    <div class="column is-5 has-text-right">
                        <div class="title is-5">{&self.job.start} {" - "} {&self.job.end}</div>
                        <div class="subtitle is-6">{"Bucharest, Romania"}</div>
                    </div>
                </div>
                <div class={classes!("columns", "is-centered", "is-mobile", menu_class)}>
                    {show_button!(Story)}
                    {show_button!(Tech)}
                    {show_button!(Resp)}
                </div>
                {show_text!(Story, show_story(), story_div)}
                {show_text!(Tech, show_tech(), tech_div)}
                {show_text!(Resp, show_resp(), resp_div)}
            </div>
        }
    }
}

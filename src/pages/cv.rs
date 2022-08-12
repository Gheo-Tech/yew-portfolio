use crate::components::job::{Job, JobCard};
use yew::prelude::*;

const CV_URI: &str = "https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/Gheorghe_Ungureanu_Resume_13-08-2022-00-21-55.pdf";

struct Education<'a> {
    title: &'a str,
    institution: &'a str,
    graduation: &'a str,
}

static EDUCATION: &[&Education] = &[
    &Education {
        title: "Master's Degree in CyberSecurity",
        institution: "Universtitatea Titu Maiorescu",
        graduation: "2015",
    },
    &Education {
        title: "Bachlor's Degree in Computer Science",
        institution: "Universtitatea Titu Maiorescu",
        graduation: "2013",
    },
    &Education {
        title: "High School, Baccalauréat",
        institution: "Colegiul National I.L. Caragiale",
        graduation: "2009",
    },
];

impl Education<'_> {
    fn render(&self) -> Html {
        html! {
            <div class="box" style="background-color: #000000">
                <div class="columns">
                    <div class="column">
                        <div class="title is-5" style="color: #bfbfbf">{&self.title}</div>
                        <div class="subtitle is-6" style="color: #bfbfbf">{&self.institution}</div>
                    </div>
                    <div class="column has-text-right">
                        <div class="title is-5" style="color: #bfbfbf">{&self.graduation}</div>
                        <div class="subtitle is-6" style="color: #bfbfbf">{"Bucharest, Romania"}</div>
                    </div>
                </div>
            </div>
        }
    }
}

pub struct CV;
impl Component for CV {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 1200px">
                <div class="title is-4 container has-text-centered">
                    {"Curriculum Vitae"}
                    <div class="jobtext" style="font-size: 14px">
                        <a href={CV_URI}>{"download pdf"}</a></div>
                </div>
                < JobCard job={Job::GheoTech} />
                < JobCard job={Job::Kape} />
                < JobCard job={Job::Vodafone} />
                < JobCard job={Job::IBM} />
                < JobCard job={Job::Freelance} />
                <br/>
                <div>{EDUCATION.iter().map(|e| e.render()).collect::<Html>()}</div>
            </div>
        }
    }
}

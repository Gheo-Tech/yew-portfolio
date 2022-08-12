use yew::prelude::*;

#[derive(PartialEq)]
pub enum Project {
    GheoTech,
    MerkleTree,
    RustPhantom,
    ActixServer,
}

pub struct ProjectMeta {
    title: String,
    description: String,
    demo: Option<String>,
    source: String,
    img: String,
}

impl From<&Project> for ProjectMeta {
    fn from(p: &Project) -> Self {
        macro_rules! match_p {
            ( $( $project_key:ident => {
                    $title:expr, $desc:expr, $demo: expr, $source: expr, $img:  expr
            }, )+
            ) => { match p {
                $( Project::$project_key => ProjectMeta {
                    title: $title.to_string(),
                    description: $desc.to_string(),
                    demo: $demo,
                    source: $source.to_string(),
                    img: $img.to_string(),
                }, )+
            }  };
        }
        match_p!(
            GheoTech => {
                "This Website",
                "Normally developers will create their portofolio website using React.
                    In my case, however, I thought that a Rust dev should use a Rust framework.
                    Because of this, the tech stack used is Yew (Rust) + Bulma (html/css).
                    Feel free to fork!",
                None,
                "https://github.com/Gheo-Tech/yew-portfolio",
                "https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/gheotech.png"
            },
            MerkleTree => {
                "Merkle Tree Demo",
                "Graphical representation of a Merkle Tree, created with Yew.
                    Helps at gaining a better understand how a blockchain works.
                    Allows you to add transactions and also to highlight the
                    hashes required to validate a transaction.",
                Some("https://gheo-tech.github.io/yew-merkle-tree".to_string()),
                "https://github.com/Gheo-Tech/yew-merkle-tree",
                "https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/merkeltree.png"
            },
            RustPhantom => {
                "Rust - Phantom POC",
                "Small POC showcasing how Seed (a Rust front-end framework) can be used
                    to interact with the Phantom Wallet (for the Solana chain). This technology
                    can be used as an alternative to TypeScript/React to create dApps.",
                Some("https://gheo.tech/rust-phantom-poc/".to_string()),
                "https://github.com/Gheo-Tech/rust-phantom-poc",
                "https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/seedphantom.png"
            },
            ActixServer => {
                "Actix API for MongoDB",
                "Actix WebServer simulating the blockchain part of a DeFi game. It was written
                    via Test Driven Development and features Table Driven Testing based on
                    declarative macros. I love the result so I am showcasing it whenever
                    I have the chance.",
                None,
                "https://github.com/Gheo-Tech/sacred-queens/tree/main/demo/server/",
                "https://filedn.com/lRvVNpEzu7mVLW5g3Ak9iOk/sacred_queens_demo_server.png"
            },
        )
    }
}

pub struct ProjectCard {
    project: ProjectMeta,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub project: Project,
}

impl Component for ProjectCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            project: ProjectMeta::from(&ctx.props().project),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let play_button = match &self.project.demo {
            Some(s) => html! {
                <a target="_blank" href={format!("{}", s)}>
                    <button class="button is-white is-small is-outlined">
                        {"▶ play"}</button>
                </a>
            },
            None => html! {},
        };
        html! {
            <button class="box has-text-centered projectbox">
                <div class="columns">
                    <div class="column">
                        <div class="subtitle is-5">{&self.project.title}</div>
                        <p class="subtitle is-italic is-6">{&self.project.description}</p>
                        <div class="columns has-text-centered is-mobile">
                            <div class="column">
                                { play_button }
                            </div>
                            <div class="column">
                                <a target="_blank" href={format!("{}", &self.project.source)}>
                                    <button class="button is-white is-small is-outlined">
                                        {"≺⁄≻ code"}
                                    </button>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="column">
                        <img class="projectimg" src={format!("{}", &self.project.img)}/>
                    </div>
                </div>
            </button>
        }
    }
}

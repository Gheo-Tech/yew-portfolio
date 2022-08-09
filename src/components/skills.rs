use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;

pub enum Msg {
    Search,
    SortByCateg,
    SortByTech,
    SortByLevel,
    SetSort(Sort),
}

#[derive(Eq, PartialEq, Clone)]
pub struct Level(usize);

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "✫".repeat(self.0.min(5)))
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Skill {
    category: String,
    technology: String,
    level: Level,
    note: String,
}

#[derive(Eq, PartialEq, Properties)]
pub struct SkillCardProps {
    skill: Rc<Skill>,
    search: Rc<String>,
}

pub struct SkillCard {
    visible: bool,
}

impl Component for SkillCard {
    type Message = ();
    type Properties = SkillCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { visible: true }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx
            .props()
            .skill
            .technology
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        if ctx
            .props()
            .skill
            .category
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        if ctx
            .props()
            .skill
            .note
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        self.visible = false;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let skill_class = match self.visible {
            true => "skill-visible",
            false => "skill-hidden",
        };
        html! {
            <div class={classes!("columns", "is-mobile", "is-gapless", "is-marginless", skill_class)}>
              <div class="column is-6"><div class="columns is-gapless is-marginless skill-group">
                <div class="column is-7 skill-category">{ctx.props().skill.category.clone()}</div>
                <div class="column is-5 skill-text">{ctx.props().skill.technology.clone()}</div>
              </div></div>
              <div class="column is-6"><div class="columns is-gapless is-marginless">
                <div class="column is-3 is-size-6 skill-level">{ctx.props().skill.level.clone()}</div>
                <div class="column is-9 gray4 has-text-right skill-desc">{ctx.props().skill.note.clone()}</div>
              </div></div>
            </div>
        }
    }
}

#[derive(PartialEq)]
pub enum Sort {
    ByCateg,
    ByTech,
    ByLevel,
}

pub struct SkillList {
    skills: Vec<Rc<Skill>>,
    search: Rc<String>,
    search_input: NodeRef,
    sort: Sort,
}

impl Component for SkillList {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SkillList {
            skills: skills_vector(),
            search: Rc::new("".to_string()),
            search_input: NodeRef::default(),
            sort: Sort::ByLevel,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        macro_rules! sort {
            ($key:ident) => {
                if self.sort != Sort::$key {
                    let ms = match *self.search == "".to_string() {
                        true => 400,
                        false => 310,
                    };
                    self.search = Rc::new("DELETE_ALL_ITEMS".to_string());
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        TimeoutFuture::new(ms).await;
                        link.send_message(Msg::SetSort(Sort::$key));
                    });
                }
            };
        }
        match msg {
            Msg::Search => {
                self.search = Rc::new(
                    self.search_input
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value(),
                );
            }
            Msg::SortByCateg => {
                sort!(ByCateg);
            }
            Msg::SortByTech => {
                sort!(ByTech);
            }
            Msg::SortByLevel => {
                sort!(ByLevel);
            }
            Msg::SetSort(s) => {
                self.sort = s;
                match self.sort {
                    Sort::ByCateg => self.skills.sort_by(|a, b| a.category.cmp(&b.category)),
                    Sort::ByTech => self.skills.sort_by(|a, b| a.technology.cmp(&b.technology)),
                    Sort::ByLevel => self.skills.sort_by(|a, b| b.level.0.cmp(&a.level.0)),
                }
                ctx.link().send_message(Msg::Search);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        macro_rules! show_table_head {
            ($class:expr, $msg:expr, $sort:expr, $text:expr) => {{
                html! {
                    <div class={$class}>
                        <a class="has-text-light has-text-weight-bold"
                        onclick={ctx.link().callback(|_| Msg::$msg)}>
                            {$text}
                            if self.sort == Sort::$sort {
                                {" ▾"}
                            }
                        </a>
                    </div>
                }
            }};
        }
        html! {
            <div class="container has-text-left" style="max-width: 1030px">
                <div class="has-text-centered"><input
                    class="input is-rounded is-white has-text-centered is-small is-overlay"
                    style="max-width: 250px"
                    placeholder="Type to search..."
                    type="text" ref={self.search_input.clone()}
                    onkeyup={ctx.link().callback(|_| Msg::Search)}
                /></div>
                <br/>
                <div style="border-bottom: 2px solid #000000; background-color: #000f0f;
                            padding: 5px 15px 5px 15px; border-radius: 20px">
                  <div class="columns is-mobile is-gapless is-marginless is-clearfix">
                     <div class="column is-6"><div class="columns is-gapless is-marginless skill-group">
                        {show_table_head!("column is-7 skill-category", SortByCateg, ByCateg, "category")}
                        {show_table_head!("column is-5 skill-title", SortByTech, ByTech, "technology")}
                     </div></div>
                     <div class="column is-6"><div class="columns is-gapless is-marginless">
                        {show_table_head!("column is-3 skill-level", SortByLevel, ByLevel, "level")}
                        <div class="column is-9 gray4 skill-desc">
                            <div class="has-text-weight-bold has-text-right">{"note"}</div>
                        </div>
                     </div></div>
                  </div>
                </div>
                <div style="padding: 0px 15px 0px 15px">
                    { for self.skills.iter().map(|s|
                        html! { <SkillCard skill={s.clone()} search={self.search.clone()}/> }
                    ) }
                </div>
            </div>
        }
    }
}

fn skills_vector() -> Vec<Rc<Skill>> {
    macro_rules! skills_vec {
            ($({ $category:literal, $name:literal, $level:literal, $note:literal },)*) => {
                vec![
                    $(
                        Rc::new(Skill {
                            category: $category.to_string(),
                            technology: $name.to_string(),
                            level: Level($level),
                            note: $note.to_string(),
                        }),
                    )*
                ]
            };
        }
    skills_vec! {
        { "Programming", "Rust", 5, "my favorite language at then moment" },
        { "Programming, Web, FrontEnd", "Rust: Yew", 5, "this website runs on Yew via WASM" },
        { "Programming, BackEnd", "Rust: Actix", 5, "the best API framework ever made" },
        { "Programming", "git", 5, "all projects must start with a git repo" },
        { "Containers, Packaging", "Podman", 5, "just use this instead of Docker" },
        { "Containers, Packaging", "Docker", 5, "I prefer Podman" },
        { "VPN, Networking", "WireGuard", 5, "deployed it to prod for multiple companies" },
        { "VPN, Networking", "OpenVPN", 5, "had to work a lot with it cause it's popular" },
        { "VPN, Networking", "IPsec, StrongSwan", 5, "really hard to master; I prefer WireGuard" },
        { "Operating Systems", "GNU/Linux", 5, "Yes, please!" },
        { "Scripting, DevOps", "Bash", 5, "forever the best scripting language" },
        { "Smart Contracts, Blockchain", "Vyper, EVM", 5, "it's great for ethereum-compatible chains" },
        { "Programming, BackEnd", "gRPC", 5, "not sure if my favorite protocol or not" },
        { "Programming, BackEnd", "RESTful", 5, "what your API usually ends up looking like" },
        { "Linux Networking, Security", "firewall, iptables", 5, "I am kinda good at hacking this" },
        { "Linux Networking", "namespaces, netns", 5, "worked a lot with namespaces" },
        { "Linux Networking", "tcpdump", 5, "is mandatory for troubleshooting stuff" },
        { "Linux Security, Cryptography", "GPG", 5, "I use this every day" },
        { "Linux Security, Cryptography", "OpenSSL", 5, "had to set up a lot of certificates" },
        { "Linux Security", "OpenSSH", 5, "people should study this a bit more" },
        { "DNS Servers", "bind9", 5, "not easy to use; still does the job" },
        { "DNS Servers", "dnsmasq", 5, "easy to use; great for MITM" },
        { "DNS Servers", "resolved", 5, "easy to use; does the job" },
        { "Linux Distribution", "ArchLinux", 5, "my favorite distro; btw, I use arch" },
        { "Linux Distribution", "Fedora", 5, "probably the best distro in the world" },
        { "Linux Distribution", "Fedora Silverblue", 5, "best desktop security ever" },
        { "Linux Distribution", "Fedora CoreOS", 5, "simply amazing for containers" },
        { "Linux Distribution", "Ubuntu", 5, "not my first pick, but it's really popular" },
        { "Linux Distribution", "Debian", 5, "I should like this more than Ubuntu, right?" },
        { "FrontEnd, Web", "HTML, CSS, Bulma", 5, "these technologies are easy to use" },
        { "Public Cloud", "AWS", 5, "worked with evil corp more than I wanted to" },
        { "Cloud Networking", "AWS", 5, "don't know why I find this easy" },
        { "Virtualization, Bare Metal", "QEMU/KVM", 5, "by far the best hypervizor in the world" },
        { "Programming, Blockchain", "Rust: Substrate", 4, "amazing for smart contracts and chains" },
        { "Programming, Blockchain", "Rust: ethers/web3", 4, "perfect WASM alternative to JavaScript" },
        { "Smart Contracts, Blockchain", "Rust: Solana", 4, "it's awesome, but I prefer Substrate" },
        { "Public Cloud", "Digital Ocean", 4, "it's cool when you need something small" },
        { "Public Cloud", "GCP", 4, "I almost accepted it as necessary evil" },
        { "Public Cloud, MicroServices", "Kubernetes", 4, "is usually my first pick for prod systems" },
        { "Scripting, DevOps", "Lua", 4, "easy to use but I don't really like it" },
        { "Web Servers, Load Balancer", "apache2", 4, "have been using it for 10 years" },
        { "Web Servers, Load Balancer", "nginx", 4, "my first pick when in need of a web server" },
        { "MicroServices, Tracing", "Jaeger", 4, "I created a POC and it looks awesome" },
        { "Packaging", "dpkg, apt", 4, "worked a lot with Ubuntu" },
        { "Packaging", "pacman", 4, "I use it a lot; btw I use Arch" },
        { "Packaging", "rpm, dnf", 4, "I like Fedora a lot" },
        { "Packaging", "rpm-ostree", 4, "this project is just amazing" },
        { "Programming, Web, FrontEnd", "Rust: Sycamore", 4, "exceptionally cool WASM framework" },
        { "Programming, BackEnd", "GoLang", 4, "my ex; used to be 5 stars" },
        { "Linux Networking", "wireshark", 4, "when tcpdump is not enough, get the big gun" },
        { "Automation, DevOps", "Ansible", 4, "Used to be my main skill some years ago." },
        { "Linux Security, Cryptography", "dmcrypt", 4, "I worked a lot with encrypted systems" },
        { "Linux Security", "SELinux", 4, "I used this on Fedora for hardening" },
        { "Load Balancer", "haproxy", 4, "helped me a lot in multiple situations" },
        { "Linux Distribution", "RHEL", 4, "not my first pick when it comes to distros" },
        { "Cloud Networking", "GCP", 4, "I like this more than the AWS alternative" },
        { "DevOps", "GitLab CI/CD", 3, "had to set up multiple projects" },
        { "DNS Servers", "PowerDNS", 3, "overcomplicated but sometimes required" },
        { "EMail Servers", "dovecot, IMAP", 3, "would be cool to work on another project" },
        { "Linux Distribution", "Gentoo", 3, "had to migrate CyberGhost away from Gentoo" },
        { "EMail Servers", "postfix, SMTP", 3, "would be cool to work on another project" },
        { "EMail Servers", "roundcube", 3, "it's easy to set up and does the job" },
        { "Cloud Networking", "Kubernetes CNI", 3, "I have good understanding of this" },
        { "Programming, BackEnd", "GraphQL", 3, "didn't work with this protocol too much" },
        { "Programming, BackEnd", "PHP", 3, "was my favorite language in 2014" },
        { "Programming, Web, FrontEnd", "JavaScript", 3, "avoid this like fire; use Rust instead" },
        { "Programming, Web, FrontEnd", "JS/TS: VueJS", 3, "easy framework but I prefer WASM+Rust" },
        { "Web Servers, Load Balancer", "Kong", 3, "I tried it out and it looks cool" },
        { "MicroServices, Service Mesh", "Istio", 3, "I read a lot but never saw it in prod" },
        { "Virtualization", "VirtualBox", 3, "you should use QEMU/KVM instead" },
        { "Smart Contracts, Blockchain", "Solidity, EVM", 3, "I prefer to use Vyper instead" },
        { "Virtualization, Bare Metal", "VMware ESXi", 3, "please just use QEMU/KVM instead" },
        { "Virtualization", "VMware vCenter", 3, "wasted 3 years of my life with this" },
        { "Scripting, BackEnd", "Python", 3, "who doesn't know Python?" },
        { "Proxy, MITM", "Squid", 3, "it's garbage; wrote my own proxy/MITM in Go" },
        { "Programming", "Java", 2, "it makes me sad when I remember I used it" },
        { "Programming", "C", 2, "it's nice to see what history looks like" },
        { "Programming", "C++", 2, "had to study it in the University" },
        { "Programming, MVC", "PHP: Laravel", 2, "overcomplicated; just use microservices" },
        { "Programming, MVC", "PHP: CakePHP", 2, "didn't touch it in the last decade" },
        { "Programming, MVC", "Python: Django", 2, "did a few projects in 2016" },
        { "Packaging", "portage, emerge", 2, "containers can satisfy the same requirement" },
        { "Linux Distribution", "NixOS", 2, "it's cool, just didn't need it yet" },
        { "Automation, DevOps", "Terraform", 2, "looks nice but we never really met" },
        { "Programming, Blockchain", "Go: Cosmos SDK", 2, "looks nice but I prefer substrate." },
        { "Private Cloud, Bare Metal", "Kubernetes", 2, "would be nice to run it in prod bare-metal" },
        { "Private Cloud, Bare Metal", "OpenStack", 2, "sadly it's not as popular as it should be" },
        { "Programming, Web, FrontEnd", "JS/TS: React", 2, "I can confidently read the code" },
        { "Programming, Web, FrontEnd", "TypeScript", 2, "it's better than JavaScript, I guess..." },
        { "Cloud Networking", "BGP, EVPN", 2, "did a POC a few years ago" },
        { "Cloud Networking", "Openstack Neutron", 2, "I should read the docs again" },
        { "DevOps", "GitHub CI/CD", 2, "Never really got a chance to try it out" },
        { "FrontEnd, Web", "Bootstrap", 2, "I prefer Bulma" },
        { "Linux Networking, Security", "firewall, ebtables", 2, "I should migrate away from iptables" },
        { "FrontEnd, Web", "Tailwind", 2, "I should invest more time into this" },
        { "Linux Security", "AppArmor", 2, "I prefer SELinux" },
        { "Operating Systems", "FreeBSD", 2, "looks awesome but never had the opportunity" },
        { "Operating Systems", "Mac", 2, "it's cool, I guess..." },
        { "Operating Systems", "Windows", 1, "No. Just no." },
        { "Public Cloud", "Azure", 1, "No. Just no." },
        { "Virtualization, Bare Metal", "HyperV", 1, "No. Just no." },
    }
}

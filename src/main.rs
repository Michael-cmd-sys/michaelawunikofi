use dioxus::prelude::*;
use js_sys::wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    description: &'static str,
    tech_stack: Vec<&'static str>,
    link: Option<&'static str>,
    status: Option<&'static str>,
}

impl Project {
    fn all() -> Vec<Self> {
        vec![
            Project {
                name: "SIF",
                description: "Accounting management software for small businesses. Streamlines financial tracking, invoicing, and reporting with a focus on clarity and ease of use.",
                tech_stack: vec!["lando", "Drupal", "Php", "PostgreSQL"],
                link: None,
                status: Some("In Active Development"),
            },
            Project {
                name: "ANHB",
                description: "Mobile ordering app for Asian Noodle House Belconnen. Features real-time order management, Square payment integration, and push notifications.",
                tech_stack: vec!["Flutter", "Firestore", "Square API", "Cloud Run"],
                link: None,
                status: Some("Live"),
            },
            Project {
                name: "Voice",
                description: "Privacy-first voice notes journal with local transcriptions. Features a custom Rust native module for enhanced audio quality, ensuring accurate speech-to-text.",
                tech_stack: vec!["React Native", "Expo", "Rust (Native Module)", "Whisper", "Speech Recognition"],
                link: None,
                status: Some("In Active Development"),
            },
            Project {
                name: "Chirpy",
                description: "Comprehensive backend API service with complete authentication, authorization, and database management. Built for scalability with Docker containerization. (Project from bootdotdev)",
                tech_stack: vec!["Go", "Docker", "JWT", "PostgreSQL"],
                link: None,
                status: Some("In Active Development"),
            },
            Project {
                name: "FunkyBeez",
                description: "Modern website uplift for Funky Hot Dogs. Features an intuitive CMS, online ordering with Square integration, and a vibrant, playful aesthetic.",
                tech_stack: vec!["Next.js", "motion", "Square API", "Vercel"],
                link: Some("https://funkybeez.com"),
                status: Some("Live"),
            },
        ]
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { href: TAILWIND_CSS, rel: "stylesheet" }

        Navbar {}
        div {
            class: "page-shell",
            Hero {}
            About {}
            Projects {}
            TechStack {}
            Contact {}
        }
        Footer {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 z-50 w-full border-b border-[color:rgba(229,224,216,0.65)] bg-[color:rgba(254,252,249,0.85)] px-6 py-5 backdrop-blur-xl md:px-8",
            div {
                class: "max-w-5xl mx-auto flex justify-between items-center",
                a {
                    href: "#",
                    class: "font-serif font-bold text-2xl text-gray-900 no-underline",
                    "MA"
                }
                div {
                    class: "flex gap-8",
                    a { href: "#about", class: "text-gray-600 no-underline text-sm font-medium hover:text-amber-700 transition-colors", "About" }
                    a { href: "#projects", class: "text-gray-600 no-underline text-sm font-medium hover:text-amber-700 transition-colors", "Projects" }
                    a { href: "#contact", class: "text-gray-600 no-underline text-sm font-medium hover:text-amber-700 transition-colors", "Contact" }
                }
            }
        }
    }
}

#[component]
fn ScrollReveal(class: String, children: Element) -> Element {
    let mut visible = use_signal(|| false);
    let id = use_hook(|| {
        let now = js_sys::Date::now();
        format!("scroll-reveal-{}", now as u64)
    });
    let id_for_effect = id.clone();

    use_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        if let Some(element) = document.get_element_by_id(&id_for_effect) {
            let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
                for i in 0..entries.length() {
                    if let Ok(entry) = entries
                        .get(i)
                        .dyn_into::<web_sys::IntersectionObserverEntry>()
                    {
                        if entry.is_intersecting() {
                            *visible.write() = true;
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            let observer =
                web_sys::IntersectionObserver::new(callback.as_ref().unchecked_ref()).ok();
            if let Some(obs) = observer {
                obs.observe(&element);
            }
            callback.forget();
        }
    });

    let class = class.clone();
    let id = id.clone();
    let is_visible = visible();
    let animated_class = if is_visible { "animate-fade-in-up" } else { "" };

    rsx! {
        div {
            id: "{id}",
            class: "{class} {animated_class}",
            {children}
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        ScrollReveal {
            class: "w-full",
            section {
                id: "hero",
                class: "site-section relative flex min-h-screen items-center justify-center overflow-hidden pt-32 pb-20 md:pt-36",
                div {
                    class: "max-w-2xl mx-auto text-center relative z-10",
                    p { class: "text-sm text-amber-700 font-medium tracking-widest uppercase mb-4", "Hello, I'm" }
                    h1 { class: "font-serif text-5xl md:text-6xl lg:text-7xl text-gray-900 leading-tight mb-2", "Michael Kofi Awuni" }
                    p { class: "text-lg text-amber-700 font-semibold tracking-wide mb-6", "Software Craftsman" }
                    p { class: "text-gray-600 max-w-md mx-auto mb-10", "Obsessed with quality software. Building elegant solutions that stand the test of time." }
                    div {
                        class: "flex flex-wrap justify-center gap-4",
                        a {
                            href: "#projects",
                            class: "px-8 py-4 bg-gray-900 text-white rounded-full font-semibold hover:bg-amber-700 hover:-translate-y-1 transition-all duration-300",
                            "View My Work"
                        }
                        a {
                            href: "#contact",
                            class: "px-8 py-4 bg-transparent text-gray-900 rounded-full font-semibold border border-stone-300 hover:border-amber-700 hover:text-amber-700 hover:-translate-y-1 transition-all duration-300",
                            "Get In Touch"
                        }
                    }
                }
                div {
                    class: "absolute inset-0 flex items-center justify-center pointer-events-none z-0",
                    div { class: "hero-wave hero-wave-secondary" }
                    div { class: "hero-wave hero-wave-primary" }
                }
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        ScrollReveal {
            class: "w-full",
            section {
                id: "about",
                class: "panel-section flex flex-col items-center justify-center",
                div {
                    class: "mb-14 text-center md:mb-16",
                    span { class: "section-kicker", "About" }
                    h2 { class: "section-heading", "The Craftsman Behind the Code" }
                }
                div {
                    class: "max-w-2xl mx-auto text-center",
                    p { class: "prose-lead", "I'm a software developer who believes great software is invisible—it just works, effortlessly." }
                    p { class: "prose-body", "With experience across mobile, web, and backend development, I approach every project with a craftsman's dedication to quality. Whether building accounting software, mobile ordering apps, or backend APIs, I focus on creating intuitive experiences that users love without even thinking about." }
                    p { class: "prose-body", "My journey has taken me from React Native and Flutter for mobile, through Go and Rust for backend systems, to modern frameworks like Dioxus. But technology aside, what drives me is simple: solving real problems with elegant solutions." }
                }
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    let projects = Project::all();

    rsx! {
        ScrollReveal {
            class: "w-full",
            section {
                id: "projects",
                class: "site-section",
                div {
                    class: "mb-16 text-center",
                    span { class: "section-kicker", "Projects" }
                    h2 { class: "section-heading", "Selected Work" }
                }
                div {
                    class: "grid grid-cols-1 justify-items-center gap-7 md:grid-cols-2 md:gap-8 lg:grid-cols-3",
                    for (i, project) in projects.iter().enumerate() {
                        ProjectCard { project: project.clone(), index: i }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project, index: usize) -> Element {
    rsx! {
        article {
            class: "project-card",
            div {
                class: "mb-5 flex items-start justify-between gap-4",
                h3 { class: "pr-2 font-serif text-xl text-gray-900", "{project.name}" }
                if let Some(status) = project.status {
                    span {
                        class: if status == "In Active Development" { "status-chip-active" } else { "status-chip-live" },
                        "{status}"
                    }
                }
            }
            p { class: "flex-grow text-sm leading-7 text-[var(--color-copy)]", "{project.description}" }
            div {
                class: "mt-8 flex flex-wrap items-center justify-center gap-3.5",
                for tech in project.tech_stack {
                    span { class: "project-pill", "{tech}" }
                }
            }
            if let Some(link) = project.link {
                a {
                    href: link,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "mt-8 inline-flex items-center gap-2 text-sm font-semibold text-amber-700 hover:text-amber-900",
                    "Visit Site"
                    span { class: "transition-transform duration-200 hover:translate-x-0.5", "→" }
                }
            }
        }
    }
}

#[component]
fn TechStack() -> Element {
    let categories = [
        ("Mobile", vec!["React Native", "Flutter", "Expo"]),
        (
            "Backend",
            vec!["Go", "Rust", "Node.js", "MongoDB", "PostgreSQL", "Docker"],
        ),
        ("Frontend", vec!["Next.js", "Dioxus", "TypeScript"]),
        ("Tools", vec!["Git", "AWS", "Firestore", "Square API"]),
    ];

    rsx! {
        ScrollReveal {
            class: "w-full",
            section {
                id: "techstack",
                class: "site-section pt-8",
                div {
                    class: "mb-16 text-center",
                    span { class: "section-kicker", "Skills" }
                    h2 { class: "section-heading", "Tech Arsenal" }
                }
                div {
                    class: "grid grid-cols-1 items-stretch justify-items-center gap-6 sm:grid-cols-2 lg:grid-cols-4 lg:gap-7",
                    for (category, tools) in categories.iter() {
                        div {
                            class: "skill-category-card",
                            h4 { class: "mb-6 text-xs font-semibold uppercase tracking-widest text-gray-500", "{category}" }
                            div {
                                class: "skill-pill-wrap",
                                for tool in tools {
                                    span { class: "skill-pill", "{tool}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Contact() -> Element {
    rsx! {
        ScrollReveal {
            class: "w-full",
            section {
                id: "contact",
                class: "panel-section text-center",
                div {
                    class: "mb-16 text-center",
                    span { class: "section-kicker", "Contact" }
                    h2 { class: "section-heading", "Let's Work Together" }
                }
                p {
                    class: "mx-auto mb-12 max-w-xl text-base leading-8 text-[var(--color-copy)] md:mb-14",
                    "I'm always open to discussing new projects, creative ideas, or opportunities to be part of your visions."
                }
                div {
                    class: "mb-10 flex flex-wrap justify-center gap-5",
                    a {
                        href: "mailto:michaelawunikofi@gmail.com",
                        class: "contact-tile",
                        div { class: "contact-icon", "✉" }
                        span { "michaelawunikofi@gmail.com" }
                    }
                    a {
                        href: "https://linkedin.com/in/michael-kofi-awuni",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-tile",
                        div { class: "contact-icon", "in" }
                        span { "LinkedIn" }
                    }
                    a {
                        href: "https://github.com/Michael-cmd-sys",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-tile",
                        div { class: "contact-icon", "</>" }
                        span { "GitHub" }
                    }
                }
                p {
                    class: "text-sm text-gray-500",
                    "Or call me: "
                    a {
                        href: "tel:+2332025029472",
                        class: "text-amber-700 font-medium hover:underline",
                        "+233 20 250 2947"
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    let year = 2026;

    rsx! {
        footer {
            class: "mx-auto w-full max-w-5xl border-t border-[var(--color-line)] px-6 py-10 text-center md:px-8 md:py-12",
            p {
                class: "mb-2 text-sm text-[var(--color-muted)]",
                "© {year} Michael Kofi Awuni. Built with care."
            }
            p {
                class: "text-xs tracking-[0.03em] text-[var(--color-muted)]",
                "Built with "
                span { class: "font-semibold text-[var(--color-accent-warm)]", "Dioxus" }
                " + "
                span { class: "font-semibold text-[var(--color-accent-warm)]", "Rust" }
            }
        }
    }
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {   

    view! { cx,
         <section>
            <h1>"Hey! I'm Jakub and this is my website"</h1>
            <img src="./assets/profile.png" alt="do zrobienia"/><br></br>
            "I'm a StarCraft 2 player for CSO Esports and currently I'm learning Rust with Leptos"<br></br>
            "and I'm planning to learn Dart and Flutter to become Full Stack Programmer" <br></br>
            "I don't have any degree I'm learning for myself only"
            <aside>
            <div id="email"></div>
            </aside>
            <div class="Socialetc"> 
                <div class="Discord">"Discord"</div>
                <div class="Twitter">"Twitter"</div>
                <div class="Twitch">"Twitch"</div>
            </div>
        </section>
        
    }
}

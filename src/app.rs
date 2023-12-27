use leptos::*;
use leptos_meta::*;
use leptos_router::*;



#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="TESTE SIMPLES WEBRUST"/>
        
        <NavBar/>
 
        // content for this pagewelcome pages 
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/About/" view=About/>
                    <Route path="/Contact/" view=Contact/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}


#[component]
fn HomePage() -> impl IntoView {

    view! {
        <>
              <section>
        <h2>Welcome to Your Website</h2>
        <p>Explore the amazing features we offer.</p>
        <button>Get Started</button>
    </section>

    <section>
        <h2>Key Features</h2>
        <p>Feature 1 description here.</p>
        <p>Feature 2 description here.</p>
        <p>Feature 3 description here.</p>
    </section>

    <section>
        <h2>Testimonials</h2>
        <blockquote>
            <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p>
            <cite>John Doe</cite>
        </blockquote>
        <blockquote>
            <p>Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
            <cite>Jane Smith</cite>
        </blockquote>
    </section>

           </>
    }
    
}

#[component]
fn About() -> impl IntoView {
    view! {
        <Style>
            include_str!("./styles/about.css");
        </Style>
        <>
            <h1>"About Us"</h1>
            <p>"We are a team passionate about Rust and web development. This is a simple web application built with the leptos framework."</p>
        </>
    }
}


/// Contact page
#[component]
fn Contact() -> impl IntoView {
    view! {
        <Style>
            include_str!("./styles/contact.css");
        </Style>
        <h1>"Contact"</h1>
    }
} 


/// NavBar Component
#[component]
fn NavBar() -> impl IntoView {
    
    view! {
        <>
            <Style>
                {include_str!("./styles/navabar.css")}
            </Style>
       <nav class="navbar">
            <ul class="nav-list">
                <li class="nav-item"><a class="nav-link" href="/">Home</a></li>
                <li class="nav-item"><a class="nav-link" href="/About">About</a></li>
                <li class="nav-item"><a class="nav-link" href="/Contact">Contact</a></li>
            </ul>
        </nav>
        </>
    }
}



/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <NavBar/>
        <h1>"Not Found"</h1>
    }
}

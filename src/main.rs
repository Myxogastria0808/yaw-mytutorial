use yew::prelude::*;

struct Article {
    content: String,
}

#[function_component(App)]
fn app() -> Html {
    //Use State
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let increment = {
        let counter: UseStateHandle<i32> = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    //Component
    let article = vec![
        Article {
            content: String::from("Article1"),
        },
        Article {
            content: String::from("Article2"),
        },
        Article {
            content: String::from("Article3"),
        },
    ];

    let articles = article
        .iter()
        .map(|article| {
            html! {
            <p>{format!("{}", article.content)}</p>
            }
        })
        .collect::<Html>();

    html! {
    <>
        <h1>{ "Hello World" }</h1>
        <button onclick={increment}>{"+1"}</button>
        <p>{*counter}</p>
        <div>{articles}</div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

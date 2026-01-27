mod logger;
use yew::prelude::*;
use crate::logger::logger::get_logger;

#[derive(Clone, PartialEq)]
struct Video {
    title: AttrValue, 
    url: AttrValue
}

#[derive(Properties, PartialEq)]
struct  VideoListProps {
    videos: Vec<Video>
}

#[component]
fn VideoList(VideoListProps { videos }: &VideoListProps) -> Html {
    html!{
        for video in videos {
            <p> { format!("{} {}", video.title, video.url) } </p>
        }
    }
}


#[component]
fn App() -> Html {
    let videos = vec![
        Video{
            title: "title 1".into(),
            url: "url 1".into()
        },
        Video{
            title: "title 2".into(),
            url: "url 2".into()
        },
    ];
    html! {
        <>
            <h1>{"Hello, world!"}</h1>

            <div>
                <VideoList {videos}/> 
            </div>
        </>
    }
}

fn main() { 

    #[cfg(not(target_arch = "wasm32"))]{
        let mut logger = get_logger().lock().unwrap();
        logger.success("запуск не wasm");
    }

    yew::Renderer::<App>::new().render();
}

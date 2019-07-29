#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

use rocket::request::FlashMessage;
use rocket::Rocket;
use rocket_contrib::{serve::StaticFiles, templates::Template};

use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::time::Duration;

//MOVED DEFINITION TO EXTERNAL FILE track.rs - max
mod mapgen;
use mapgen::get_map;
use mapgen::track::Track;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct MyTrack {
    track_list: Vec<Track> 
}

#[post("/media/<id>")]
fn play_victory(id: &rocket::http::RawStr) -> String {
    let device = rodio::default_output_device().unwrap();
    //    let _current_song: Track = Track::new("media/victory.mp3".to_string());
    let mut path = "media/".to_string();
    path.push_str(id.as_str());
    println!("{}", path);
    let _current_song_path = Path::new(&path);
    let _current_song: Track = Track::new(_current_song_path);
    let file = std::fs::File::open(&path).unwrap();
    let victory = rodio::play_once(&device, BufReader::new(file)).unwrap();
    victory.set_volume(1.0);
    victory.play();

    println!("{}", _current_song.title);

    thread::sleep(Duration::from_millis(4500));
    _current_song.title.to_string()
}

#[get("/get_songs")]
fn get_songs() -> Json<MyTrack> {
    let _current_song_path = Path::new("media/victory.mp3");
    let _current_song: Track = Track::new(_current_song_path);
    let mut track_list: Vec<Track> = Vec::new();
    track_list.push(_current_song);
    let tracks: MyTrack = MyTrack{track_list};
    Json(tracks)
}
    


#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
        }
    }

    pub fn raw(msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context { msg: msg }
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage<'_, '_>>) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(Some((msg.name(), msg.msg()))),
            None => Context::raw(None),
        },
    )
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index, play_victory, get_songs])
        .attach(Template::fairing())
}

fn main() {
    // Example playlist entry

    let media_dir = Path::new("media/");
    let music_lib = get_map(&media_dir);

    match music_lib {
        Ok(a) => {
            println!("HashMap has {} values\n", a.len());
            for b in a.keys() {
                let track = a.get(b).unwrap();
                println!("{}", track.title);
            }
        }
        Err(_) => println!("ERROR READING MUSIC LIBRARY"),
    }

    rocket().launch();
}

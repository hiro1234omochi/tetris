
#![windows_subsystem = "windows"]
use std::thread::Scope;
use std::io;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::tetris::{BlockType, KeyType, MoveMessage, RandomGenerator};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level,debug};
use tetris::TetrisManager;
use serde::{Deserialize, Serialize};

const STYLE: &str =  /*manganis::mg!(fileasset!(*/include_str!("../assets/style.css");/* );*/
const SCRIPT: &str =  /*manganis::mg!(fileasset!(*/include_str!("../assets/script.js");
mod tetris;
pub fn human_can(f: &Arc<Mutex<Vec<Vec<BlockType>>>>)-> String{
    let mut a=String::new();
    for y in f.lock().unwrap().iter(){
        for x in y.iter(){
            match x {
                &BlockType::MinoInMotion(_)=>a+="◆",
                &BlockType::Empty => a+="□",
                &BlockType::Wall => a+="■",
                &BlockType::Obstruction => a+="■",
                &BlockType::MinoBlock(_) => a+="■",
                &BlockType::Ghost(_) => a+="◇",
            };
        }

        a+="\n"
    }
    a
}
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[derive(Serialize,Debug,PartialEq,Eq,Clone)]
struct TetrisDataContainer{
    field: Vec<Vec<BlockType>>,
    nexts:  Vec<Vec<Vec<BlockType>>>,
    hold: Option<Vec<Vec<BlockType>>>,
}
#[component]
fn App() -> Element {
    //let a = use_state(|| 0);
    let mut tetris_manager= use_signal_sync(|| tetris::TetrisManager::default());
    
    let _: Coroutine<()>=use_coroutine(|rx| async move {
        let mut previous_tetris_data_container: Option<TetrisDataContainer>=None;
        let mut is_updated=false;
        //let eval = eval(SCRIPT);
        /*let eval=eval(r#"
        while(true){
            let msg = await dioxus.recv();

            document.getElementById("a").innerHTML=msg
        }"#);*/
        let mut a=0;
        loop{
            let eval = eval(SCRIPT);
            tetris_manager.write().update();
            if tetris_manager.read().get_is_finished(){
                tetris_manager.set(tetris::TetrisManager::default());
                tetris_manager.write().update();
            }
            let data: (Vec<Vec<BlockType>>, Vec<tetris::MinoType>, Option<tetris::MinoType>)=tetris_manager.read().get_data_to_draw(7);
            let tetris_data_container=TetrisDataContainer{
                field: data.0,
                nexts: data.1.iter().map({|&mino_type|{mino_type.hold_field().iter().map(|row|{row.to_vec()}).collect()}}).collect(),
                hold: if let Some(hold) = data.2{
                    Some(hold.hold_field().iter().map(|&row|{row.to_vec()}).collect())
                }else{
                    None
                }
            };
            /*is_updated=if let Some(previous_tetris_data_container)=previous_tetris_data_container.clone(){
                previous_tetris_data_container!=tetris_data_container
            }else{
                true
            };
            if is_updated{
                eval.send(serde_json::Value::String(serde_json::to_string(&tetris_data_container).unwrap()));               
                previous_tetris_data_container=Some(tetris_data_container);
            }*/
            
            previous_tetris_data_container=Some(tetris_data_container.clone());
            
            eval.send(serde_json::Value::String(serde_json::to_string(&tetris_data_container).unwrap()));               
            
            
            //eval.send(a.into());
            /* 
            async_std::task::sleep(Duration::from_millis(10)).await;
            async_std::task::yield_now().await;
            */
            async_std::task::sleep(*tetris::SLEEP_TIME.lock().unwrap()/*std::time::Duration::from_millis(18)*/).await;
            //info!("starting app");
            //a=1
            /*
            tokio::time::sleep(Duration::from_secs(1)).await;

             */
        }
    });
    let a: (Vec<Vec<BlockType>>, Vec<tetris::MinoType>, Option<tetris::MinoType>)=
        tetris_manager.read().get_data_to_draw(7);
    let mut handle_key_event = move |evt: KeyboardEvent,is_down: bool|{
        if evt.is_auto_repeating(){
            return;
        }
        match evt.key() {
            Key::Character(character) => match character.as_str() {
                "z"=>tetris_manager.write().send_key(KeyType::RotateLeft,is_down),
                "x"=>tetris_manager.write().send_key(KeyType::RotateRight,is_down),
                "c"=>tetris_manager.write().send_key(KeyType::Hold,is_down),
                " "=>tetris_manager.write().send_key(KeyType::HardDrop,is_down),
                _=>{},
            },
            Key::ArrowLeft=>{tetris_manager.write().send_key(KeyType::Left,is_down)},
            Key::ArrowRight=>{tetris_manager.write().send_key(KeyType::Right,is_down)},
            Key::ArrowDown=>{tetris_manager.write().send_key(KeyType::SoftDrop,is_down)},
            _ => {}

        };
    };
    let datas: (Vec<Vec<BlockType>>, Vec<tetris::MinoType>, Option<tetris::MinoType>)=tetris_manager.read().get_data_to_draw(7);
    rsx! {
        link{
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/destyle.css@1.0.15/destyle.css",
        }
        /*
        style{
            "{STYLE}"
        }*/
        //head::Link{ href: STYLE, rel: "stylesheet"}
        //tetris-field=220px 400px
        canvas {
            id: "tetris",
            tabindex: "-1",
            onkeydown: move |evt|{handle_key_event(evt,true)},
            onkeyup: move |evt|{handle_key_event(evt,false)},
        }
        div { 
            id: "a"
        }

    }

}
/*
fn main()  {

    use tetris::{BlockType, Mino, MinoType};
    //let mut field = BlockType::create_empty_field();
    /*let mut field= [
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0],
        [0,0,1,0,0,1,0,0,0,0],
        [1,1,1,0,0,0,1,1,1,1],
        [1,1,1,1,0,1,1,1,1,1],
    ];
    let mut b=RandomGenerator::default();
    let mut field= BlockType::translation(
        field.iter().map(|row|{
        row.iter().map(|&block|{
            block.into()
        }).collect()
    }).collect());*/
    let mut tetrisManager=tetris::TetrisManager::default();
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line.");
        match buffer.trim().to_lowercase().as_str(){
            "l"=>tetrisManager.send_key(KeyMessage::Left(true)),
            "r"=>tetrisManager.send_key(KeyMessage::Right(true)),
            "180"=>tetrisManager.send_key(KeyMessage::Rotate180),
            "rr"=>tetrisManager.send_key(KeyMessage::RotateRight),
            "rl"=>tetrisManager.send_key(KeyMessage::RotateLeft),
            "hd"=>tetrisManager.send_key(KeyMessage::HardDrop),
            "sd"=>tetrisManager.send_key(KeyMessage::HardDrop), 
                                        "hold"=>tetrisManager.send_key(KeyMessage::Hold),
            _=>{},
        };
    }
    /*
    
    print!("{}", human_can(&field));
    let mut a = Mino::new(b.next().unwrap(), &field).unwrap();
    println!("{:?}", a.get_mino_type());
    print!("{}", human_can(&field));
    a.move_order(MoveMessage::RotateRight);
    print!("{}", human_can(&field));
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line.");
        let c=match buffer.trim().to_lowercase().as_str(){
            "left"=>a.move_order(MoveMessage::Left).is_ok(),
            "right"=>a.move_order(MoveMessage::Right).is_ok(),
            "180"=>a.move_order(MoveMessage::Rotate180).is_ok(),
            "rr"=>a.move_order(MoveMessage::RotateRight).is_ok(),
            "rl"=>a.move_order(MoveMessage::RotateLeft).is_ok(),
            "fix"=>a.move_order(MoveMessage::Fix).is_ok(),
            "down"=>a.move_order(MoveMessage::Down).is_ok(),
            _=>false,
        };
        if (a.is_fixed()){
            a=Mino::new(b.next().unwrap(), &mut field).unwrap();
        }
        print!("{}\n\n", human_can(&field));

        print!("{}\n\n\n",c);
        print!("{}\n\n\n",b.clone().take(8).map(|x| {
            format!("{:?}",x)
        }).collect::<Vec<_>>().join(" "));
    }*/
}*/


#![windows_subsystem = "windows"]
use std::thread::Scope;
use std::io;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use crate::tetris::{BlockType, KeyType, MoveMessage, RandomGenerator};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level,debug};
use tetris::TetrisManager;

const STYLE: &str =  /*manganis::mg!(fileasset!(*/include_str!("../assets/style.css");/* );*/
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
    #[cfg(features="web")]
    console_error_panic_hook::set_once();
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}
#[component]
fn App() -> Element {
    //let a = use_state(|| 0);
    let mut tetris_manager= use_signal_sync(|| tetris::TetrisManager::default());
    let _: Coroutine<()>=use_coroutine(|rx| async move {
        loop{
            tetris_manager.write().update();
            if tetris_manager.read().get_is_finished(){
                tetris_manager.set(tetris::TetrisManager::default());
                tetris_manager.write().update();
            }
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
        style{
            "{STYLE}"
        }
        //head::Link{ href: STYLE, rel: "stylesheet"}

        div{ 
            id: "tetris",
            onkeydown: move |evt|{handle_key_event(evt,true)},
            onkeyup: move |evt|{handle_key_event(evt,false)},
            tabindex: "-1",
            div{
                id: "hold",
                if let Some(mino_type)=datas.2{
                    for column in mino_type.hold_field(){
                        div{  
                            for block in column{
                                div {
                                    class: "tetris_cell {block.get_class_name()}" 
                                }
                            }
                        }
                    }
                }else{
                    for _ in 0..2{
                        div{  
                            for _ in 0..4{
                                div {
                                    class: "tetris_cell Empty"
                                }
                            }
                        }
                    }
                }
            }
            div{
                id: "field",       
                for column in datas.0{
                    div{  
                        for block in column{
                            div {
                                class: "tetris_cell {block.get_class_name()}" 
                            }
                        }
                    }
                }
            }

            div{
                id: "nexts",
                for next in datas.1{
                    div{
                        for column in next.hold_field(){
                            div{  
                                for block in column{
                                    div {
                                        class: "tetris_cell {block.get_class_name()}" 
                                    }
                                }
                            }
                        }
                    }
                }
            }
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

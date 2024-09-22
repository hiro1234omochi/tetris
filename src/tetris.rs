use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicUsize, Ordering};
use std::thread;
use dioxus::events::keyboard_types::KeyState;
use serde::Serialize;
use web_time::{Duration, Instant};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use MinoDirection::{East, North, South, West};
use MinoType::*;
use crate::human_can;

pub static WIDTH: AtomicUsize = AtomicUsize::new(10);
pub static HEIGHT: AtomicUsize = AtomicUsize::new(26);
pub static APPEARANCE_POSITION:(AtomicI64,AtomicI64) = (AtomicI64::new(3),AtomicI64::new(5));

//static DCD: i64 = 0;
pub static ARR: Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(15)))});
pub static DAS: Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(200)))});
pub static SOFT_DROP_DISTANCE: Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::ZERO))});
pub static GRAVITY_DISTANCE:Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(1000)))});
pub static LOCKDOWN_DISTANCE:Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(500)))});
pub static SLEEP_TIME:Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(1000/120)))});
//static CHECK_IS_HITTING_GROUND_DISTANCE:Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(10)))});
//static CHECK_DAS_DISTANCE:Lazy<Arc<Mutex<Duration>>>=Lazy::new(|| {Arc::new(Mutex::new(Duration::from_millis(10)))});
pub static MOVE_COUNT_LIMIT:AtomicUsize= AtomicUsize::new(15);

#[allow(clippy::type_complexity)]
static ROTATIONS: Lazy<HashMap<MinoType, HashMap<MinoDirection, [[i64; 4]; 4]>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    result.insert(MinoT, HashMap::from([
        (North, [
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (East, [
            [0, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ]),
        (South, [
            [0, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ]),
        (West, [
            [0, 1, 0, 0],
            [1, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ])
    ]));
    result.insert(MinoS, HashMap::from([
        (North, [
            [0, 1, 1, 0],
            [1, 1, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (East, [
            [0, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 0]
        ]),
        (South, [
            [0, 0, 0, 0],
            [0, 1, 1, 0],
            [1, 1, 0, 0],
            [0, 0, 0, 0]
        ]),
        (West, [
            [1, 0, 0, 0],
            [1, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ])
    ]));
    result.insert(MinoZ, HashMap::from([
        (North, [
            [1, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (East, [
            [0, 0, 1, 0],
            [0, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ]),
        (South, [
            [0, 0, 0, 0],
            [1, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        (West, [
            [0, 1, 0, 0],
            [1, 1, 0, 0],
            [1, 0, 0, 0],
            [0, 0, 0, 0]
        ])
    ]));
    result.insert(MinoL, HashMap::from([
        (North, [
            [0, 0, 1, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (East, [
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        (South, [
            [0, 0, 0, 0],
            [1, 1, 1, 0],
            [1, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (West, [
            [1, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ])
    ]));
    result.insert(MinoJ, HashMap::from([
        (North, [
            [1, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]),
        (East, [
            [0, 1, 1, 0],
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 0, 0]
        ]),
        (South, [
            [0, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 0]
        ]),
        (West, [
            [0, 1, 0, 0],
            [0, 1, 0, 0],
            [1, 1, 0, 0],
            [0, 0, 0, 0]
        ])
    ]));
    result.insert(MinoO, HashMap::from([
        (North, [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]),
        (East, [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]),
        (South, [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]),
        (West, [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]])
    ]));
    result.insert(MinoI, HashMap::from([
        (North, [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]]),
        (East, [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]]),
        (South, [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]),
        (West, [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]])
    ]));
    result
});
//回転前から回転後にhashmap
//https://harddrop.com/wiki/SRS
//https://tetris.fandom.com/wiki/SRS

#[allow(clippy::type_complexity)]
static OFFSETS: Lazy<HashMap<MinoDirection, HashMap<MinoDirection, [(i64, i64); 5]>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    result.insert(North, HashMap::from(
        [
            (
                East, [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            ),
            (
                West, [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            )
        ]
    ));
    result.insert(East, HashMap::from(
        [
            (
                South, [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
            ),
            (
                North, [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
            )
        ]
    ));
    result.insert(South, HashMap::from(
        [
            (
                West, [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
            ),
            (
                East, [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
            )
        ]
    ));
    result.insert(West, HashMap::from(
        [
            (
                North, [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            ),
            (
                South, [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
            )
        ]
    ));
    result
});

#[allow(clippy::type_complexity)]
static OFFSETS_MINO_L: Lazy<HashMap<MinoDirection, HashMap<MinoDirection, [(i64, i64); 5]>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    result.insert(North, HashMap::from([
        (
            East, [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
        ), (
            West, [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
        )
    ]));
    result.insert(East, HashMap::from([
        (
            South, [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
        ), (
            North, [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
        )
    ]));
    result.insert(South, HashMap::from([
        (
            West, [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
        ), (
            East, [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
        )
    ]));
    result.insert(West, HashMap::from([
        (
            North, [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
        ), (
            South, [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
        )
    ]));
    result
});

#[allow(clippy::type_complexity)]
static OFFSETS_180: Lazy<HashMap<MinoDirection, HashMap<MinoDirection, [(i64, i64); 11]>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    result.insert(North, HashMap::from(
        [(
            South, [(1, 0), (2, 0), (1, 1), (2, 1), (-1, 0), (-2, 0), (-1, 1), (-2, 1), (0, -1), (3, 0), (-3, 0)]
        )]
    ));
    result.insert(East, HashMap::from(
        [
            (
                West, [(0, 1), (0, 2), (-1, 1), (-1, 2), (0, -1), (0, -2), (-1, -1), (-1, -2), (1, 0), (0, 3), (0, -3)]
            )
        ]
    ));
    result.insert(South, HashMap::from(
        [
            (
                North, [(-1, 0), (-2, 0), (-1, -1), (-2, -1), (1, 0), (2, 0), (1, -1), (2, -1), (0, 1), (-3, 0), (3, 0)]
            ),
        ]
    ));
    result.insert(West, HashMap::from(
        [
            (
                East, [(0, 1), (0, 2), (1, 1), (1, 2), (0, -1), (0, -2), (1, -1), (1, -2), (-1, 0), (0, 3), (0, -3)]
            )
        ]
    ));
    result
});

#[allow(clippy::type_complexity)]
static OFFSETS_MINO_L_180: Lazy<HashMap<MinoDirection, HashMap<MinoDirection, [(i64, i64); 5]>>> = Lazy::new(|| {
    let mut result = HashMap::new();
    result.insert(North, HashMap::from([
        (
            South, [(-1, 0), (-2, 0), (1, 0), (2, 0), (0, 1)]
        )
    ]));
    result.insert(East, HashMap::from([
        (
            West, [(0, 1), (0, 2), (0, -1), (0, -2), (-1, 0)]
        )
    ]));
    result.insert(South, HashMap::from([
        (
            North, [(1, 0), (2, 0), (-1, 0), (-2, 0), (0, -1)]
        )
    ]));
    result.insert(West, HashMap::from([
        (
            East, [(0, 1), (0, 2), (0, -1), (0, -2), (1, 0)]
        )
    ]));
    result
});

const MINO_ARRAY: [MinoType; 7]=[MinoI, MinoO, MinoS, MinoZ, MinoJ, MinoL, MinoT];
const KEY_TYPE_ARRAY: [KeyType; 8]=[
    KeyType::Left,
    KeyType::Right,
    KeyType::RotateRight,
    KeyType::RotateLeft,
    KeyType::Rotate180,
    KeyType::HardDrop,
    KeyType::SoftDrop,
    KeyType::Hold
];
pub enum MoveMessage {
    Left,
    Right,
    Down,
    RotateRight,
    RotateLeft,
    Rotate180,
    Fix,
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyType {
    Left,
    Right,
    RotateRight,
    RotateLeft,
    Rotate180,
    HardDrop,
    SoftDrop,
    Hold
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum KeyStatePlus{
    DownUnprocessed,
    DownProcessed,
    Up,
}
#[derive(Serialize,Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MinoType {
    MinoI,
    MinoO,
    MinoS,
    MinoZ,
    MinoJ,
    MinoL,
    MinoT,
}
impl MinoType{
    pub fn to_string(&self)->String{
        match(self){
            MinoI=>"MinoI",
            MinoO=>"MinoO",
            MinoS=>"MinoS",
            MinoZ=>"MinoZ",
            MinoJ=>"MinoJ",
            MinoL=>"MinoL",
            MinoT=>"MinoT",
        }.to_string()
    }
    pub fn hold_field(&self)->[[BlockType;4];2]{
        if *self==MinoO{
            [[BlockType::Empty,BlockType::MinoBlock(MinoO),BlockType::MinoBlock(MinoO),BlockType::Empty];2]
        }else{
            ROTATIONS.get(self).unwrap().get(&North).unwrap()[0..2].iter().map(|row|->[BlockType;4]{
                row.iter().map(|&is_some|{
                    if is_some==0{
                        BlockType::Empty
                    }else{
                        BlockType::MinoBlock(*self)
                    }
                }).collect::<Vec<BlockType>>().try_into().unwrap()
            }).collect::<Vec<[BlockType;4]>>().try_into().unwrap()
        }
    }
}
#[derive(Eq, PartialEq, Hash, Clone, Copy,Debug)]
pub enum MinoDirection {
    North,
    East,
    South,
    West,
}
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct RandomGenerator{
    stock: Vec<MinoType>,
    min_items: usize,
}
impl RandomGenerator{
    fn get_size_generated(&self) -> usize{
        self.stock.len()
    }
    fn get_items(&mut self,size: usize) -> Vec<MinoType>{
        self.generate(size);
        self.stock.clone()[0..size].to_vec()
    }
    fn generate(&mut self, min_size: usize){
        while self.stock.len() < min_size {
            let mut mino_array=MINO_ARRAY.to_vec();
            let mut rng = rand::thread_rng();
            mino_array.shuffle(&mut rng);
            self.stock.extend(mino_array);
        }
    }
}
impl Default for RandomGenerator {
    fn default() -> Self {
        Self{stock: Vec::new(),min_items: 20}
    }
}
impl Iterator for RandomGenerator {
    type Item = MinoType;
    fn next(&mut self) -> Option<Self::Item> {
        /*
        while self.stock.len() < self.min_items {
            let mut mino_array=MINO_ARRAY.to_vec();
            let mut rng = rand::thread_rng();
            mino_array.shuffle(&mut rng);
            self.stock.extend(mino_array);
        }*/
        self.generate(self.min_items);
        let result = self.stock[0];
        self.stock.remove(0);
        //popの最初からみたいな感じ
        Some(result)
    }
}
struct TetrisVariableContainer{
    mino: Mino,
    hold: Option<MinoType>,
    next: Option<MinoType>,
    is_hold_executed: bool,
    last_down_executed: Option<Instant>,
    start_left_pressing: Option<Instant>,
    start_right_pressing: Option<Instant>,
    start_waiting_das: Option<Instant>,
    arr_direction_is_right: Option<bool>,
    //last_check_das_executed: Option<Instant>,
    last_arr_execute: Option<Instant>,
    is_in_soft_drop: bool,
    last_soft_drop_execute: Option<Instant>,
    last_move_execute: Option<Instant>,
    move_count_since_lockdown_execute: usize,
    // last_check_if_hitting_ground: Option<Instant>,
    first_hitting_ground: Option<Instant>,
}
pub struct TetrisManager{
    is_finished: bool,
    key_states: Arc<Mutex<HashMap<KeyType,KeyStatePlus>>>,
    field: Arc<Mutex<Vec<Vec<BlockType>>>>,
    random_generator: Arc<Mutex<RandomGenerator>>,
    hold_mino: Arc<Mutex<Option<MinoType>>>,
    tetris_variable_container: TetrisVariableContainer,
}
impl TetrisManager {
    pub fn get_is_finished(&self)->bool{
        self.is_finished
    }
    pub fn send_key(&mut self, key_type: KeyType,is_down: bool) {
        self.key_states.lock().unwrap().insert(key_type,if is_down{KeyStatePlus::DownUnprocessed}else{KeyStatePlus::Up});
    }
    pub fn get_data_to_draw(&self, preview_size: usize) -> (Vec<Vec<BlockType>>, Vec<MinoType>, Option<MinoType>) {
        (self.field.lock().unwrap().clone(), self.random_generator.lock().unwrap().get_items(preview_size), if let Some(hold) = *self.hold_mino.lock().unwrap() {
            Some(hold)
        } else {
            None
        })
    }
    pub fn update(&mut self) {
        if self.is_finished{
            return;
        }
            let soft_drop_distance = SOFT_DROP_DISTANCE.lock().unwrap();
            let gravity_distance = GRAVITY_DISTANCE.lock().unwrap();
            //let check_is_hitting_ground_distance = CHECK_IS_HITTING_GROUND_DISTANCE.lock().unwrap();
            //let check_das_distance = CHECK_DAS_DISTANCE.lock().unwrap();
            let lockdown_distance = LOCKDOWN_DISTANCE.lock().unwrap();
            let sleep_time = SLEEP_TIME.lock().unwrap();
            let das = DAS.lock().unwrap();
            let arr = ARR.lock().unwrap();
            let now_instant = Instant::now();
            let mut is_hold_executed_this_moment=false;
            let previous_field=self.field.lock().unwrap().clone();
            //dioxus_logger::tracing::info!("{:?}",self.key_states.lock().unwrap());
            for key_type in  KEY_TYPE_ARRAY{
                let previous_minimum_y = self.tetris_variable_container.mino.minimum_y;
                let &key_state= self.key_states.lock().unwrap().get(&key_type).unwrap();
                let is_moved = match key_type{
                    KeyType::Hold=>{
                        if key_state==KeyStatePlus::DownUnprocessed{
                            if self.tetris_variable_container.is_hold_executed {
                            } else {
                                (self.tetris_variable_container.hold, self.tetris_variable_container.next) = if let Some(hold) = self.tetris_variable_container.hold {
                                    (Some(self.tetris_variable_container.mino.mino_type), Some(hold))
                                } else {
                                    (Some(self.tetris_variable_container.mino.mino_type), self.random_generator.lock().unwrap().next())
                                };
                                *self.hold_mino.lock().unwrap() = self.tetris_variable_container.hold;
                                is_hold_executed_this_moment=true;
                            }
                        }
                        false
                    },
                    KeyType::HardDrop => {
                        if key_state==KeyStatePlus::DownUnprocessed{
                            while self.tetris_variable_container.mino.move_order(MoveMessage::Down).is_ok() {}
                            self.tetris_variable_container.mino.fix();
                            self.tetris_variable_container.next = self.random_generator.lock().unwrap().next();
                        }
                        false
                    },
                    KeyType::RotateLeft=> {
                        if key_state==KeyStatePlus::DownUnprocessed{
                            self.tetris_variable_container.mino.move_order(MoveMessage::RotateLeft).is_ok()
                        }else{
                            false
                        }
                    },
                    KeyType::RotateRight => {
                        if key_state==KeyStatePlus::DownUnprocessed{
                            self.tetris_variable_container.mino.move_order(MoveMessage::RotateRight).is_ok()
                        }else{
                            false
                        }
                    },
                    KeyType::Rotate180=> {
                        if key_state==KeyStatePlus::DownUnprocessed{
                            self.tetris_variable_container.mino.move_order(MoveMessage::Rotate180).is_ok()
                        }else{
                            false
                        }
                    },
                    KeyType::Left => {
                        if key_state==KeyStatePlus::DownUnprocessed {
                            self.tetris_variable_container.start_left_pressing = Some(now_instant);
                            self.tetris_variable_container.mino.move_order(MoveMessage::Left).is_ok()
                        } else if key_state==KeyStatePlus::Up{
                            self.tetris_variable_container.start_left_pressing = None;
                            false
                        }else{
                            false
                        }
                    },
                    KeyType::Right => {
                        if key_state==KeyStatePlus::DownUnprocessed{
                            self.tetris_variable_container.start_right_pressing = Some(now_instant);
                            self.tetris_variable_container.mino.move_order(MoveMessage::Right).is_ok()
                        } else if key_state==KeyStatePlus::Up{
                            self.tetris_variable_container.start_right_pressing = None;
                            false
                        }else{
                            false
                        }
                    },
                    KeyType::SoftDrop => {
                        self.tetris_variable_container.is_in_soft_drop = (key_state!=KeyStatePlus::Up);
                        false //便宜上
                    }
                };
                if key_state==KeyStatePlus::DownUnprocessed{
                    self.key_states.lock().unwrap().insert(key_type,KeyStatePlus::DownProcessed);
                }
                if is_moved {
                    self.tetris_variable_container.last_move_execute = Some(now_instant);
                    self.tetris_variable_container.move_count_since_lockdown_execute += 1; //下に落ちたときとかハードドロップとかは下の処理で0にされるから気にしてない
                }
                if previous_minimum_y < self.tetris_variable_container.mino.minimum_y { //minimum_yの値が大きくなったということは下に行ったということである。
                    self.tetris_variable_container.move_count_since_lockdown_execute = 0;
                    self.tetris_variable_container.first_hitting_ground = None;
                }
            }
            { //判定系
                if self.tetris_variable_container.is_in_soft_drop {
                    let previous_minimum_y = self.tetris_variable_container.mino.minimum_y;
                    if soft_drop_distance.is_zero() { //infinityつまり即時着地
                        while self.tetris_variable_container.mino.move_order(MoveMessage::Down).is_ok() {}
                    } else if self.tetris_variable_container.last_soft_drop_execute.is_none() || now_instant - self.tetris_variable_container.last_soft_drop_execute.unwrap() > *soft_drop_distance {
                        let _ = self.tetris_variable_container.mino.move_order(MoveMessage::Down);
                    }
                    if previous_minimum_y < self.tetris_variable_container.mino.minimum_y { //minimum_yの値が大きくなったということは最低点を更新したってこと
                        self.tetris_variable_container.move_count_since_lockdown_execute = 0;
                        self.tetris_variable_container.first_hitting_ground = None;
                    }
                    self.tetris_variable_container.last_soft_drop_execute = Some(now_instant);
                }
                //if self.tetris_variable_container.last_check_das_executed.is_none() || now_instant - self.tetris_variable_container.last_soft_drop_execute.unwrap() > *check_das_distance {
                if (self.tetris_variable_container.start_left_pressing.is_some() && self.tetris_variable_container.start_right_pressing.is_some()) || (self.tetris_variable_container.start_left_pressing.is_none() && self.tetris_variable_container.start_right_pressing.is_none()) {
                    self.tetris_variable_container.start_waiting_das = None;
                    self.tetris_variable_container.arr_direction_is_right = None;
                } else {
                    if self.tetris_variable_container.start_waiting_das.is_none(){
                        self.tetris_variable_container.start_waiting_das = Some(now_instant);
                    }
                    self.tetris_variable_container.arr_direction_is_right = Some(self.tetris_variable_container.start_right_pressing.is_some());
                }
                //}
                if (self.tetris_variable_container.start_waiting_das.is_some() && now_instant - self.tetris_variable_container.start_waiting_das.unwrap() > *das){
                    if (self.tetris_variable_container.last_arr_execute.is_none() || now_instant - self.tetris_variable_container.last_arr_execute.unwrap() > *arr) {
                        if self.tetris_variable_container.mino.move_order(if self.tetris_variable_container.arr_direction_is_right.unwrap() { MoveMessage::Right } else { MoveMessage::Left }).is_ok()
                        { self.tetris_variable_container.move_count_since_lockdown_execute += 1;}
                        self.tetris_variable_container.last_arr_execute = Some(now_instant);
                    }
                }
                if self.tetris_variable_container.last_down_executed.is_none() || now_instant - self.tetris_variable_container.last_down_executed.unwrap() > *gravity_distance { //普通に落ちる時間がどうかの判定
                    let previous_minimum_y = self.tetris_variable_container.mino.minimum_y;
                    let _ = self.tetris_variable_container.mino.move_order(MoveMessage::Down);
                    if previous_minimum_y < self.tetris_variable_container.mino.minimum_y { //minimum_yの値が大きくなったということは最低点を更新したってこと
                        self.tetris_variable_container.move_count_since_lockdown_execute = 0;
                        self.tetris_variable_container.first_hitting_ground = None;
                    }
                    self.tetris_variable_container.last_down_executed = Some(now_instant);
                }
                //if self.tetris_variable_container.last_check_if_hitting_ground.is_none() || now_instant - self.tetris_variable_container.last_check_if_hitting_ground.unwrap() > *check_is_hitting_ground_distance {
                if self.tetris_variable_container. mino.is_hitting_ground() { //地面に接触しているか否か
                    if self.tetris_variable_container.first_hitting_ground.is_none(){
                        self.tetris_variable_container.first_hitting_ground = Some(now_instant)
                    } //値が入っていればそのまま
                } else {
                    self.tetris_variable_container.first_hitting_ground = None;
                }
                    //self.tetris_variable_container.last_check_if_hitting_ground = Some(now_instant);
                //}
                if self.tetris_variable_container.first_hitting_ground.is_some() && now_instant - self.tetris_variable_container.first_hitting_ground.unwrap() > *lockdown_distance { //最初に地面に接触してから一定の時間が経ったら
                    if self.tetris_variable_container.last_move_execute.is_none() || now_instant - self.tetris_variable_container.last_move_execute.unwrap() > *lockdown_distance { //最後に動かされてから一定時間が経過しているか、一定回数以上動いたら
                        self.tetris_variable_container.mino.fix();
                        self.tetris_variable_container.next = self.random_generator.lock().unwrap().next(); //次のミノにうつる
                    } 
                }
                if  self.tetris_variable_container.move_count_since_lockdown_execute >= MOVE_COUNT_LIMIT.load(Ordering::Relaxed) && self.tetris_variable_container.first_hitting_ground.is_some(){
                    self.tetris_variable_container.mino.fix();
                    self.tetris_variable_container.next = self.random_generator.lock().unwrap().next(); //次のミノにうつる
                }
                if(*self.field.lock().unwrap()!=previous_field){
                    self.tetris_variable_container.mino.draw_ghost();
                }
            }
            if let Some(next) = self.tetris_variable_container.next {
                let mino_result = Mino::new(next, &self.field.clone());
                self.tetris_variable_container.next=None;
                if is_hold_executed_this_moment{
                    self.tetris_variable_container.is_hold_executed = true;
                }else{
                    self.tetris_variable_container.is_hold_executed = false;
                }
                self.tetris_variable_container.last_down_executed = None;
                self.tetris_variable_container.last_move_execute = None;
                self.tetris_variable_container.move_count_since_lockdown_execute = 0;
                //self.tetris_variable_container.last_check_if_hitting_ground = None;
                self.tetris_variable_container.first_hitting_ground = None;
                //変数のinit soft_drop arr dasなどは除く
                if let Ok(mino_result) = mino_result {
                    self.tetris_variable_container.mino=mino_result;
                    //dioxus_logger::tracing::info!("{}",self.tetris_variable_container.mino.is_hitting_ground());
                    //dioxus_logger::tracing::info!("{:?}",self.tetris_variable_container.mino);
                    let field= self.field.lock().unwrap().clone();
                    for row in field.iter().enumerate(){
                        if row.1.iter().find(|block|{!block.has_collision_detection()}).is_none(){
                            
                            self.field.lock().unwrap().remove(row.0);
                            self.field.lock().unwrap().insert(0,vec![BlockType::Empty; WIDTH.load(Ordering::Relaxed)]);
                        }
                    }
                    //8todo!("line消去プログラム")
                } else {
                    self.is_finished=true;
                }
            }
            //println!("{:?}", human_can(&self.field.clone()));
        
    }
}
impl Default for TetrisManager {
    fn default() -> Self {
        let key_states=Arc::new(Mutex::new(HashMap::new()));
        for key_type in KEY_TYPE_ARRAY{
            key_states.lock().unwrap().insert(key_type, KeyStatePlus::Up);
        }
        let field_mutex = Arc::new(Mutex::new(BlockType::create_empty_field()));
        let random_generator = Arc::new(Mutex::new(RandomGenerator::default()));
        let hold_mino = Arc::new(Mutex::new(None));
        let tetris_variable_container = TetrisVariableContainer{
             mino: Mino::new(random_generator.lock().unwrap().next().unwrap(), &field_mutex.clone()).unwrap() ,
             hold: None,
             next: None,
             is_hold_executed: false,
             last_down_executed: None,
             start_left_pressing: None,
             start_right_pressing: None,
             start_waiting_das: None,
             arr_direction_is_right: None,
             //last_check_das_executed: None,
             last_arr_execute: None,
             is_in_soft_drop: false,
             last_soft_drop_execute: None,
             last_move_execute: None,
             move_count_since_lockdown_execute: 0,
             //last_check_if_hitting_ground: None,
             first_hitting_ground: None,
        };
        /*
        let thread = thread::spawn(
            {
                let key_set=key_set.clone();
                let field_mutex=field_mutex.clone();
                let random_generator=random_generator.clone();
                let hold_mino=hold_mino.clone();
                move || {
                    let mut mino=Mino::new(random_generator.lock().unwrap().next().unwrap(),&field_mutex).unwrap();
                    let mut hold: Option<MinoType> =None;
                    let mut next:Option<MinoType> = None;
                    let mut is_hold_executed:bool=false;
                    let mut last_down_executed:Option<Instant> = None;
                    let mut start_left_pressing:Option<Instant> = None;
                    let mut start_right_pressing:Option<Instant> = None;
                    let mut start_waiting_das:Option<Instant> = None;
                    let mut arr_direction_is_right:Option<bool> = None;
                    let mut last_check_das_executed:Option<Instant> = None;
                    let mut last_arr_execute:Option<Instant> = None;
                    let mut is_in_soft_drop:bool=false;
                    let mut last_soft_drop_execute:Option<Instant> = None;
                    let mut last_move_execute:Option<Instant> = None;
                    let mut move_count_since_lockdown_execute:usize=0;
                    let mut last_check_if_hitting_ground:Option<Instant>=None;
                    let mut first_hitting_ground:Option<Instant>=None;
                    loop {
                        let soft_drop_distance = SOFT_DROP_DISTANCE.lock().unwrap();
                        let gravity_distance=GRAVITY_DISTANCE.lock().unwrap();
                        let check_is_hitting_ground_distance=CHECK_IS_HITTING_GROUND_DISTANCE.lock().unwrap();
                        let check_das_distance=CHECK_DAS_DISTANCE.lock().unwrap();
                        let lockdown_distance=LOCKDOWN_DISTANCE.lock().unwrap();
                        let sleep_time=SLEEP_TIME.lock().unwrap();
                        let das=DAS.lock().unwrap();
                        let arr=ARR.lock().unwrap();
                        let mut key_set = key_set.lock().unwrap();
                        let now_instant=Instant::now();
                        for key_message in key_set.iter(){
                            let previous_minimum_y=mino.minimum_y;
                            let is_moved =match key_message {
                                KeyMessage::Hold=>{
                                    if is_hold_executed {
                                        //false
                                    }else {
                                        (hold, next) = if let Some(hold)=hold{
                                            (Some(mino.mino_type), Some(hold))
                                        } else {
                                            (Some(mino.mino_type), random_generator.lock().unwrap().next())
                                        };
                                        *hold_mino.lock().unwrap() = hold;
                                        is_hold_executed=true;
                                        //true
                                    }
                                    false
                                },
                                KeyMessage::HardDrop=>{
                                    while mino.move_order(MoveMessage::Down).is_ok() {}
                                    next=random_generator.lock().unwrap().next();
                                    false
                                },
                                KeyMessage::RotateLeft=>{
                                    mino.move_order(MoveMessage::RotateLeft).is_ok()
                                },
                                KeyMessage::RotateRight=>{
                                    mino.move_order(MoveMessage::RotateRight).is_ok()
                                },
                                KeyMessage::Rotate180=>{
                                    mino.move_order(MoveMessage::Rotate180).is_ok()
                                },
                                KeyMessage::Left(b)=>{
                                    if *b {
                                        start_left_pressing = Some(now_instant);
                                        mino.move_order(MoveMessage::Left).is_ok()
                                    }else{
                                        start_left_pressing=None;
                                        false
                                    }
                                },
                                KeyMessage::Right(b)=>{
                                    if *b {
                                        start_right_pressing = Some(now_instant);
                                        mino.move_order(MoveMessage::Left).is_ok()
                                    }else{
                                        start_left_pressing=None;
                                        false
                                    }
                                },
                                KeyMessage::SoftDrop(b)=>{
                                    is_in_soft_drop=*b;
                                    false//便宜上
                                }
                            };
                            if is_moved {
                                last_move_execute=Some(now_instant);
                                move_count_since_lockdown_execute+=1;//下に落ちたときとかハードドロップとかは下の処理で0にされるから気にしてない
                            }
                            if previous_minimum_y<mino.minimum_y {//minimum_yの値が大きくなったということは下に行ったということである。
                                move_count_since_lockdown_execute=0;
                                first_hitting_ground=None;
                            }
                        }
                        key_set.clear();//処理されたものは削除
                        {//判定系
                            if is_in_soft_drop {
                                if soft_drop_distance.is_zero() { //infinityつまり即時着地
                                    while mino.move_order(MoveMessage::Down).is_ok() {}
                                } else if last_soft_drop_execute.is_none() || now_instant - last_soft_drop_execute.unwrap() > *soft_drop_distance {
                                    let _ = mino.move_order(MoveMessage::Down);
                                    last_soft_drop_execute = Some(now_instant);
                                }
                            }
                            if last_check_das_executed.is_none() || now_instant - last_soft_drop_execute.unwrap() > *check_das_distance {
                                if (start_left_pressing.is_some() && start_right_pressing.is_some()) || (start_left_pressing.is_none() && last_soft_drop_execute.is_none()) {
                                    start_waiting_das = None;
                                    arr_direction_is_right = None;
                                } else {
                                    start_waiting_das = Some(now_instant);
                                    arr_direction_is_right = Some(start_right_pressing.is_some());
                                }
                            }
                            if (start_waiting_das.is_some() && now_instant - start_waiting_das.unwrap() > *das) && (last_arr_execute.is_none() || now_instant - last_arr_execute.unwrap() > *arr) {
                                let _ = mino.move_order(if arr_direction_is_right.unwrap() { MoveMessage::Right } else { MoveMessage::Left });
                                last_arr_execute = Some(now_instant);
                            }
                            if last_down_executed.is_none() || now_instant - last_down_executed.unwrap() > *gravity_distance { //普通に落ちる時間がどうかの判定
                                let previous_minimum_y = mino.minimum_y;
                                let _ = mino.move_order(MoveMessage::Down);
                                if previous_minimum_y < mino.minimum_y { //minimum_yの値が大きくなったということは最低点を更新したってこと
                                    move_count_since_lockdown_execute = 0;
                                    first_hitting_ground = None;
                                }
                                last_down_executed = Some(now_instant);
                            }
                            if last_check_if_hitting_ground.is_none() || now_instant - last_check_if_hitting_ground.unwrap() > *check_is_hitting_ground_distance {
                                if mino.is_hitting_ground() { //地面に接触しているか否か
                                    if self.tetris_variable_container.first_hitting_ground.is_none(){
                                        self.tetris_variable_container.first_hitting_ground = Some(now_instant)
                                    } //値が入っていればそのまま
                                } else {
                                    first_hitting_ground = None;
                                }
                                last_check_if_hitting_ground = Some(now_instant);
                            }
                            if first_hitting_ground.is_some() && now_instant - first_hitting_ground.unwrap() > *lockdown_distance { //最初に地面に接触してから一定の時間が経ったら
                                if last_move_execute.is_none() || now_instant - last_move_execute.unwrap() > *lockdown_distance || move_count_since_lockdown_execute >= MOVE_COUNT_LIMIT.load(Ordering::Relaxed) { //最後に動かされてから一定時間が経過しているか、一定回数以上動いたら
                                    next=random_generator.lock().unwrap().next();//次のミノにうつる
                                }
                            }
                        }
                        if let Some(next)=next{
                            let mino_result= Mino::new(next,&field_mutex);
                            is_hold_executed=false;
                            last_down_executed=None;
                            last_move_execute=None;
                            move_count_since_lockdown_execute=0;
                            last_check_if_hitting_ground=None;
                            first_hitting_ground=None;
                            //変数のinit soft_drop arr dasなどは除く
                            if let Ok(mino_result)=mino_result{
                                todo!("line消去プログラム")
                            }else{
                                println!("You lose!")
                            }
                        }
                        println!("{:?}",human_can(&field_mutex));
                        thread::sleep(*sleep_time);
                    }
                }
            }
        );*/
        Self{
            key_states,
            is_finished: false,
            field: field_mutex,
            random_generator,
            hold_mino,
            tetris_variable_container
        }
    }
}
impl MinoDirection {
    fn previous(self) -> Self {
        match &self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn next(&self) -> Self {
        match &self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum MinoState {
    InMotion,
    Fixed,
}
#[derive(Serialize,Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum BlockType {
    Empty,
    Wall,
    Obstruction,
    MinoBlock(MinoType),
    MinoInMotion(MinoType),
    Ghost(MinoType),
} 
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum RotationType {
    RotateLeft,
    RotateRight,
    Rotate180,
}
impl BlockType {
    pub fn create_empty_field() -> Vec<Vec<BlockType>> {
        let field = vec![vec![BlockType::Empty; WIDTH.load(Ordering::Relaxed)]; HEIGHT.load(Ordering::Relaxed)];
        /*
        for y in &mut field {
            y.insert(0, BlockType::Wall);
            y.insert(WIDTH - 1, BlockType::Wall);
        }
        field.insert(HEIGHT - 1, vec![BlockType::Wall; WIDTH]);
        */
        field
    }
    pub fn translation(vec: Vec<Vec<i64>>)->Vec<Vec<BlockType>> {
        let mut vec2= vec![vec![BlockType::Empty; vec[0].len()]; vec.len()];
        for (row,row2) in vec.iter().zip(vec2.iter_mut()) {
            for (&col,col2) in row.iter().zip(row2.iter_mut()) {
                if col==1{
                    *col2=BlockType::Obstruction;
                }
            }
        }

        vec2
    }
    pub fn get_class_name(&self)->String{
        match self{
            BlockType::Empty=>"Empty".to_string(),
            BlockType::Obstruction=>"Obstruction".to_string(),
            BlockType::Wall=>"Wall".to_string(),
            BlockType::MinoBlock(mino_type)=>format!("MinoBlock {}",mino_type.to_string()),
            BlockType::MinoInMotion(mino_type)=>format!("MinoInMotion {}",mino_type.to_string()),
            BlockType::Ghost(mino_type)=>format!("Ghost {}",mino_type.to_string()),
        }
    }
    fn has_collision_detection(&self) -> bool {
        matches!(self, BlockType::Wall | BlockType::Obstruction | BlockType::MinoBlock(_))
    }

}

#[derive(Clone,Debug)]
pub struct Mino {
    x: i64,
    y: i64,
    field: Arc<Mutex<Vec<Vec<BlockType>>>>,
    state: MinoState,
    mino_direction: MinoDirection,
    mino_type: MinoType,
    does_rotate: bool,
    rotation: HashMap<MinoDirection, [[i64; 4]; 4]>,
    minimum_y: i64,
}

impl Default for Mino {
    fn default() -> Self {
        Self {
            x: APPEARANCE_POSITION.0.load(Ordering::Relaxed),
            y: APPEARANCE_POSITION.1.load(Ordering::Relaxed),
            state: MinoState::InMotion,
            mino_direction: North,
            mino_type: MinoO,
            does_rotate: false,
            rotation:  ROTATIONS.get(&MinoO).unwrap().clone(),
            field: Arc::new(Mutex::new(Vec::new())),
            minimum_y: APPEARANCE_POSITION.1.load(Ordering::Relaxed),
        }
    }
}

impl Mino {
    pub fn new(mino_type: MinoType, field_arg: &Arc<Mutex<Vec<Vec<BlockType>>>>) ->Result<Self,()> {
        let default_mino = Mino::default();
        let field=field_arg.clone();
        let mut result = if mino_type == MinoO {
            Mino {
                field,
                ..default_mino
            }
        } else {
            let rotation = ROTATIONS.get(&mino_type).unwrap().clone();
            Mino {
                field,
                mino_type,
                rotation,
                does_rotate: true,
                ..default_mino
            }
        };
            result.set_appearance_position();
        if result.replace(result.x, result.y, result.mino_direction).is_ok(){
            Ok(result)
        }else{
            Err(())
        }
    }
    pub fn is_fixed(&self) -> bool {
        self.state == MinoState::Fixed
    }
    pub fn get_mino_type(&self) -> MinoType {
        self.mino_type
    }
    pub fn set_position(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }
    pub fn can_move_order(&self,move_message: MoveMessage) -> bool {
        match move_message {
            MoveMessage::RotateRight => self.can_rotate(RotationType::RotateRight),
            MoveMessage::RotateLeft => self.can_rotate(RotationType::RotateLeft),
            MoveMessage::Rotate180 => self.can_rotate(RotationType::Rotate180),
            MoveMessage::Left => self.can_replace(self.x - 1, self.y, self.mino_direction),
            MoveMessage::Right => self.can_replace(self.x + 1, self.y, self.mino_direction),
            MoveMessage::Down => {
                self.can_replace(self.x, self.y + 1, self.mino_direction)
            }
            MoveMessage::Fix => {
                self.can_fix()
            }
        }
    }
    pub fn move_order(&mut self, move_message: MoveMessage) -> Result<(),()> {
        match move_message {
            MoveMessage::RotateRight => self.rotate(RotationType::RotateRight),
            MoveMessage::RotateLeft => self.rotate(RotationType::RotateLeft),
            MoveMessage::Rotate180 => self.rotate(RotationType::Rotate180),
            MoveMessage::Left => self.replace(self.x - 1, self.y, self.mino_direction),
            MoveMessage::Right => self.replace(self.x + 1, self.y, self.mino_direction),
            MoveMessage::Down => {
                self.replace(self.x, self.y + 1, self.mino_direction)
            }
            MoveMessage::Fix => {
                self.fix()
            }
        }
    }
    pub fn is_hitting_ground(&self) -> bool {
        !self.can_move_order(MoveMessage::Down)
    }
    fn fix(&mut self) -> Result<(),()> {
        self.state = MinoState::Fixed;
        self.replace(self.x, self.y, self.mino_direction)
    }
    fn can_fix(&self) -> bool {
        self.can_replace(self.x, self.y, self.mino_direction)
    }
    //https://tetrisch.github.io/main/srs.html
    // このサイトを参考にした。
    //https://tetris.wiki/Super_Rotation_System

    fn can_rotate(&self,rotation_type: RotationType) -> bool {
        let result=self.get_offsets(rotation_type);
        if let Some(offsets) = result.0 {
            for offset in offsets {
                if self.can_replace(self.x + offset.0, self.y - offset.1, result.1){
                    return true;
                }
            }
            false
        }else{
            true//offsetがNoneってことはdoes_rotate=falseってことだから成功する
        }
    }
    fn rotate(&mut self, rotation_type: RotationType) -> Result<(),()>  {
        let result=self.get_offsets(rotation_type);
        if let Some(offsets) = result.0 {
            for offset in offsets {
                if self.replace(self.x + offset.0, self.y - offset.1, result.1).is_ok() {
                    return Ok(());
                }
            }
            Err(())
        }else{
            Ok(())//offsetがNoneってことはdoes_rotate=falseってことだから成功する
        }

    }
    #[allow(clippy::type_complexity)]
    fn get_offsets(&self, rotation_type: RotationType) -> (Option<Vec<(i64,i64)>>,MinoDirection)  {
        let expect_direction = match rotation_type {
            RotationType::RotateRight => self.mino_direction.next(),
            RotationType::RotateLeft => self.mino_direction.previous(),
            RotationType::Rotate180 => self.mino_direction.next().next(),
        };
        if !self.does_rotate {
            return (None,expect_direction) //回転する必要なし=>いつでも成功
        }
        /*
        if self.replace(self.x, self.y, expect_direction, rotation_next, field) {
            return true; //0パターン
        }*/
        let offsets: Vec<(i64,i64)> = match rotation_type {
            RotationType::RotateRight | RotationType::RotateLeft => if self.mino_type == MinoType::MinoI {
                OFFSETS_MINO_L.get(&self.mino_direction).unwrap().get(&expect_direction).unwrap().into()
            } else {
                OFFSETS.get(&self.mino_direction).unwrap().get(&expect_direction).unwrap().into()
            },
            RotationType::Rotate180 => {
                if self.mino_type == MinoType::MinoI {
                    OFFSETS_MINO_L_180.get(&self.mino_direction).unwrap().get(&expect_direction).unwrap().into()
                } else {
                    OFFSETS_180.get(&self.mino_direction).unwrap().get(&expect_direction).unwrap().into()
                }
            }
        };
        (Some(offsets),expect_direction)
        /*
        if expect_direction == West || expect_direction == East{
            let x = if expect_direction == East { self.x + 1 } else { self.x - 1 };
            { //1パターンと2パターン
                if self.replace(x, self.y,rotation_previous, rotation_next) {
                    return true;
                }
                if self.replace(x, self.y - 1,rotation_previous, rotation_next) {
                    return true;
                }
            }
            {//パターン3とパターン4
                let y= self.y + 2;
                if self.replace(self.x, y,rotation_previous, rotation_next) {
                    return true;
                }
                if self.replace(x, y,rotation_previous, rotation_next) {
                    return true;
                }
            }


        }else {
            let x = if is_right { self.x - 1 } else { self.x + 1 };
            { //パターン1と2
                if self.replace(x, self.y, rotation_previous, rotation_next) {
                    return true;
                }
                if self.replace(x, self.y + 1, rotation_previous, rotation_next) {
                    return true;
                }
            }
            { //パターン3と4
                let y = self.y - 2;
                if self.replace(self.x, y, rotation_previous, rotation_next) {
                    return true;
                }
                if self.replace(x, y, rotation_previous, rotation_next) {
                    return true;
                }
            }
        }
         */
        /*
        for offset in offsets {
            if self.replace(self.x + offset.0, self.y - offset.1, expect_direction, rotation_next).is_ok() {
                return Ok(());
            }
        }
        Err(())*/
    }
    fn replace(&mut self, x: i64, y: i64, mino_direction: MinoDirection) -> Result<(),()> {
        let result=self.replace_common(x,y,mino_direction)?;
        (self.x,self.y,self.mino_direction,*self.field.lock().unwrap())=(result.0,result.1,result.2,result.3);
        if self.minimum_y<self.y{//最低値が更新されたら更新する。
            self.minimum_y = self.y;
        }
        Ok(())
    }
    fn can_replace(&self, x: i64, y: i64, mino_direction: MinoDirection) -> bool{
        self.replace_common(x,y,mino_direction).is_ok()
    }
    #[allow(clippy::type_complexity)]
    fn replace_common(&self, x: i64, y: i64, mino_direction: MinoDirection) -> Result<(i64,i64,MinoDirection,Vec<Vec<BlockType>>),()> {
        let field=self.field.lock().unwrap();
        let &after=ROTATIONS.get(&self.get_mino_type()).unwrap().get(&mino_direction).unwrap();
        //動いてるミノ全部削除したリストを作る
        let mut field2: Vec<Vec<BlockType>> = field.iter().map(|vec| -> Vec<BlockType>{
            vec.iter().map(|block_type| {
                if let BlockType::MinoInMotion(_) = block_type {
                    BlockType::Empty
                } else {
                    *block_type
                }
            }).collect()
        }).collect();
        //dioxus_logger::tracing::info!("{:?}",field2);
        //衝突判定して、どれかが重なっていたら即終了
        for (yi, vy) in (y..y + 4).zip(after.iter()) {
            for (xi, &vx) in (x..x + 4).zip(vy) {
                if vx == 1 {
                    if !(yi >= 0 && xi >= 0) {
                        return Err(());
                    }
                    let yi = yi as usize;
                    let xi = xi as usize;

                    let Some(data) = field2.get_mut(yi) else {
                        return Err(());
                    };

                    let Some(block) = data.get(xi) else {
                        return Err(());
                    };
                    if block.has_collision_detection() {
                        return Err(());
                    }
                    data[xi] = match self.state {
                        MinoState::InMotion => BlockType::MinoInMotion(self.mino_type),
                        MinoState::Fixed => BlockType::MinoBlock(self.mino_type)
                    };
                }
            }
        }
        /*
        (self.x, self.y) = (x, y);
        self.mino_direction = mino_direction;
        *field= field2;
        if self.minimum_y<self.y{
            self.minimum_y = self.y;
        }
        */
        Ok((x,y,mino_direction,field2))
    }
    pub fn set_appearance_position(&mut self){
        (self.x,self.y)=(APPEARANCE_POSITION.0.load(Ordering::Relaxed),APPEARANCE_POSITION.1.load(Ordering::Relaxed));
    }
    pub fn draw_ghost(&mut self){
        let mut y=self.y;
        let &rotaition=ROTATIONS.get(&self.get_mino_type()).unwrap().get(&self.mino_direction).unwrap();
        while self.can_replace(self.x, y+1, self.mino_direction){
            y+=1;
        }
        let mut field=self.field.lock().unwrap();
        let mut field2: Vec<Vec<BlockType>> = field.iter().map(|vec| -> Vec<BlockType>{
            vec.iter().map(|block_type| {
                if let BlockType::Ghost(_) = block_type {
                    BlockType::Empty
                } else {
                    *block_type
                }
            }).collect()
        }).collect();   //ghostは全部削除
        
        for(iy,vy) in rotaition.iter().enumerate(){
            for(ix,&vx) in vy.iter().enumerate(){
                if vx==1{
                    let block=&mut field2[(iy as i64+y) as usize][(self.x+ix as i64) as usize];
                    match block.clone(){
                        BlockType::Ghost(_)=>{}
                        BlockType::Empty=>{}
                        _=>continue
                    }
                    *block=BlockType::Ghost(self.get_mino_type());
                }
            }
        }
        *field=field2;
        

    }
}
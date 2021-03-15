#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RlState {
    //RL
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}

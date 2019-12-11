//* 魔物
pub struct Enemy {
    //* 名前
    pub name: String,
    //* レベル
    pub level: String,
    //* 分類
    pub kind: String,
    //* 知能
    pub intelligence: String,
    //* 知覚
    pub sensation: String,
    //* 反応
    pub reaction: String,
    //* 言語
    pub language: Vec<String>,
    //* 生息地
    pub habitat: String,
    //* 知名度
    pub popularity: (String, String),
    //* 弱点
    pub weak_point: String,
    //* 先制値
    pub preemption: String,
    //* 移動速度
    pub speed: String,
    //* 生命抵抗力
    pub life_resistance: String,
    //* 精神抵抗力
    pub mental_resistance: String,
    //* 特殊能力
    pub special_ability: String,
    //* 部位
    pub parts: Vec<Part>,
    //* 戦利品
    pub rewards: Vec<Reward>,
}

//* 部位
pub struct Part {
    //* 攻撃方法
    pub way_to_attack: String,
    //* 命中
    pub accuracy: String,
    //* 打撃点
    pub damage: String,
    //* 回避力
    pub evasion: String,
    //* 防護点
    pub defense: String,
    //* HP
    pub hp: String,
    //* mp
    pub mp: String,
}

//* 戦利品
pub struct Reward {
    //* 出目
    pub dice: String,
    //* アイテム
    pub item: String,
}

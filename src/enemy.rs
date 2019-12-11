//* 魔物
#[derive(Deserialize)]
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
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct Reward {
    //* 出目
    pub dice: String,
    //* アイテム
    pub item: String,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            name: String::new(),
            level: String::new(),
            kind: String::new(),
            intelligence: String::new(),
            sensation: String::new(),
            reaction: String::new(),
            language: Vec::new(),
            habitat: String::new(),
            popularity: (String::new(), String::new()),
            weak_point: String::new(),
            preemption: String::new(),
            speed: String::new(),
            life_resistance: String::new(),
            mental_resistance: String::new(),
            special_ability: String::new(),
            parts: Vec::new(),
            rewards: Vec::new(),
        }
    }
}

impl Part {
    pub fn new() -> Self {
        Part {
            way_to_attack: String::new(),
            accuracy: String::new(),
            damage: String::new(),
            evasion: String::new(),
            defense: String::new(),
            hp: String::new(),
            mp: String::new(),
        }
    }
}

impl Reward {
    pub fn new() -> Self {
        Reward {
            dice: String::new(),
            item: String::new(),
        }
    }
}

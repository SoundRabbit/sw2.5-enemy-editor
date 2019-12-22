use crate::enemy;
use crate::enemy::Enemy;
use crate::xml::Node;

impl Node {
    fn data_with_children(name: impl Into<String>, children: Vec<Node>) -> Self {
        let mut data = Node::new("data");
        data.add_attribute("name", name);
        for child in children {
            data.add_child(child);
        }
        data
    }

    fn data_with_value(name: impl Into<String>, value: impl Into<String>) -> Self {
        let mut data = Node::new("data");
        data.add_attribute("name", name);
        data.add_value(value);
        data
    }

    fn data_as_image(value: impl Into<String>) -> Self {
        let mut data = Node::new("data");
        data.add_attribute("type", "image");
        data.add_attribute("name", "imageIdentifier");
        data.add_value(value);
        data
    }

    fn data_as_resource(name: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        let mut data = Node::new("data");
        data.add_attribute("type", "numberResource");
        data.add_attribute("currentValue", &value);
        data.add_attribute("name", name);
        data.add_value(value);
        data
    }

    fn chat_palette(value: impl Into<String>) -> Self {
        let mut data = Node::new("chat-palette");
        data.add_attribute("dicebot", "SwordWorld2_5");
        data.add_value(value);
        data
    }
}

impl Enemy {
    pub fn to_udonarium_string(&self) -> String {
        let mut character = Node::new("character");
        character.add_attribute("location.name", "table");
        character.add_attribute("location.x", "0");
        character.add_attribute("location.y", "0");
        character.add_attribute("posZ", "0");
        character.add_attribute("rotate", "0");
        character.add_attribute("roll", "0");

        let mut resources = vec![];
        let mut abilities = vec![];
        let mut chat_text = String::new();
        for part in &self.parts {
            let resource = part.to_udonarium_resource_nodes(true);
            let ability = part.to_udonarium_ability_nodes(true);
            chat_text = chat_text + &part.to_udonarium_chat_pallet(true) + "\n";
            for r in resource {
                resources.push(r);
            }
            for a in ability {
                abilities.push(a);
            }
        }

        character.add_child(Node::data_with_children(
            "character",
            vec![
                Node::data_with_children("image", vec![Node::data_as_image("")]),
                Node::data_with_children(
                    "common",
                    vec![
                        Node::data_with_value("name", &self.name),
                        Node::data_with_value("size", "1"),
                    ],
                ),
                Node::data_with_children(
                    "detail",
                    vec![
                        Node::data_with_children("リソース", resources),
                        Node::data_with_children("能力", abilities),
                        self.to_udonarium_information_node(),
                    ],
                ),
            ],
        ));

        character.add_child(Node::chat_palette(chat_text));

        character.to_string()
    }
    pub fn to_udonarium_string_with_part(&self, part: usize) -> Option<String> {
        if let Some(part) = self.parts.get(part) {
            let mut character = Node::new("character");
            character.add_attribute("location.name", "table");
            character.add_attribute("location.x", "0");
            character.add_attribute("location.y", "0");
            character.add_attribute("posZ", "0");
            character.add_attribute("rotate", "0");
            character.add_attribute("roll", "0");
            character.add_child(Node::data_with_children(
                "character",
                vec![
                    Node::data_with_children("image", vec![Node::data_as_image("")]),
                    Node::data_with_children(
                        "common",
                        vec![
                            Node::data_with_value(
                                "name",
                                String::new() + &self.name + "(" + &part.name + ")",
                            ),
                            Node::data_with_value("size", "1"),
                        ],
                    ),
                    Node::data_with_children(
                        "detail",
                        vec![
                            Node::data_with_children(
                                "リソース",
                                part.to_udonarium_resource_nodes(false),
                            ),
                            Node::data_with_children(
                                "能力",
                                part.to_udonarium_ability_nodes(false),
                            ),
                            self.to_udonarium_information_node(),
                        ],
                    ),
                ],
            ));
            character.add_child(Node::chat_palette(part.to_udonarium_chat_pallet(false)));
            Some(character.to_string())
        } else {
            None
        }
    }

    fn to_udonarium_information_node(&self) -> Node {
        Node::data_with_children(
            "情報",
            vec![
                Node::data_with_value("知能", &self.intelligence),
                Node::data_with_value("知覚", &self.sensation),
                Node::data_with_value("反応", &self.reaction),
                Node::data_with_value("言語", &self.language),
                Node::data_with_value("生息地", &self.habitat),
                Node::data_with_value(
                    "知名度",
                    String::new() + &self.popularity.0 + "/" + &self.popularity.1,
                ),
                Node::data_with_value("弱点", &self.weak_point),
                Node::data_with_value("先制値", &self.preemption),
                Node::data_with_value("移動速度", &self.speed),
                Node::data_with_value("生命抵抗力", &self.life_resistance),
                Node::data_with_value("精神抵抗力", &self.mental_resistance),
            ],
        )
    }
}

impl enemy::Part {
    fn to_udonarium_resource_nodes(&self, with_name: bool) -> Vec<Node> {
        if with_name {
            let name = String::new() + "(" + &self.name + ")";
            vec![
                Node::data_as_resource(String::new() + "HP" + &name, &self.hp),
                Node::data_as_resource(String::new() + "MP" + &name, &self.mp),
            ]
        } else {
            vec![
                Node::data_as_resource("HP", &self.hp),
                Node::data_as_resource("MP", &self.mp),
            ]
        }
    }

    fn to_udonarium_ability_nodes(&self, with_name: bool) -> Vec<Node> {
        if with_name {
            let name = String::new() + "(" + &self.name + ")";
            vec![
                Node::data_with_value(String::new() + "攻撃方法" + &name, &self.way_to_attack),
                Node::data_with_value(String::new() + "命中力" + &name, &self.accuracy),
                Node::data_with_value(String::new() + "打撃点" + &name, &self.damage),
                Node::data_with_value(String::new() + "回避力" + &name, &self.evasion),
                Node::data_with_value(String::new() + "防護点" + &name, &self.defense),
            ]
        } else {
            vec![
                Node::data_with_value("攻撃方法", &self.way_to_attack),
                Node::data_with_value("命中力", &self.accuracy),
                Node::data_with_value("打撃点", &self.damage),
                Node::data_with_value("回避力", &self.evasion),
                Node::data_with_value("防護点", &self.defense),
            ]
        }
    }

    fn to_udonarium_chat_pallet(&self, with_name: bool) -> String {
        let mut text = String::new();
        let name = if with_name {
            String::new() + "(" + &self.name + ")"
        } else {
            String::new()
        };
        let way_to_attack = String::new() + "攻撃方法" + &name;
        let accuracy = String::new() + "命中力" + &name;
        let damage = String::new() + "打撃点" + &name;
        let evasion = String::new() + "回避力" + &name;
        let defense = String::new() + "防護点" + &name;
        let hp = String::new() + "HP" + &name;
        let mp = String::new() + "MP" + &name;
        if with_name {
            text = text + "==========" + &self.name + "==========\n";
        }
        text = text
            + "2d+{"
            + &accuracy
            + "} 【命中力判定"
            + &name
            + "】攻撃方法：{"
            + &way_to_attack
            + "}\n";
        text = text
            + "{"
            + &damage
            + "} 【与ダメージ"
            + &name
            + "】攻撃方法：{"
            + &way_to_attack
            + "}\n";
        text = text + "\n";
        text = text + "2d+{" + &evasion + "} 【回避力判定" + &name + "】\n";
        text = text + "2d+{精神抵抗力} 【精神抵抗力判定" + &name + "】\n";
        text = text + "2d+{生命抵抗力} 【生命抵抗力判定" + &name + "】\n";
        text = text + "\n";
        text = text + "C({" + &hp + "}-()+{" + &defense + "}) 【残HP" + &name + " 物理ダメージ】\n";
        text = text + "C({" + &hp + "}-()) 【残HP" + &name + " 魔法ダメージ】\n";
        text = text + "C({" + &hp + "}-()/2) 【残HP" + &name + " 魔法ダメージ半減】\n";
        text = text + "C({" + &mp + "}-()) 【残MP" + &name + "】\n";
        text
    }
}

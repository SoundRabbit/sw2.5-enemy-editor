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

    fn data_as_resource(name: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        let mut data = Node::new("data");
        data.add_attribute("type", "numberResource");
        data.add_attribute("currentValue", &value);
        data.add_attribute("name", name);
        data.add_value(value);
        data
    }
}

impl Enemy {
    pub fn to_udonarium_string(&self, part: usize) -> Option<String> {
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
                            Node::data_with_children(
                                "リソース",
                                vec![
                                    Node::data_as_resource("HP", &part.hp),
                                    Node::data_as_resource("MP", &part.mp),
                                ],
                            ),
                            Node::data_with_children(
                                "能力",
                                vec![
                                    Node::data_with_value("攻撃方法", &part.way_to_attack),
                                    Node::data_with_value("命中", &part.accuracy),
                                    Node::data_with_value("打撃点", &part.damage),
                                    Node::data_with_value("回避力", &part.evasion),
                                    Node::data_with_value("防護点", &part.defense),
                                ],
                            ),
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
                                        String::new()
                                            + &self.popularity.0
                                            + "/"
                                            + &self.popularity.1,
                                    ),
                                    Node::data_with_value("弱点", &self.weak_point),
                                    Node::data_with_value("先制値", &self.preemption),
                                    Node::data_with_value("移動速度", &self.speed),
                                    Node::data_with_value("生命抵抗力", &self.life_resistance),
                                    Node::data_with_value("精神抵抗力", &self.mental_resistance),
                                ],
                            ),
                        ],
                    ),
                ],
            ));
            Some(character.to_string())
        } else {
            None
        }
    }
}

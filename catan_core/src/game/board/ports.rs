use crate::game::PortType;

#[derive(Debug)]
pub struct Port {
    pub port_type: PortType,
    pub building_idx: i32,
}

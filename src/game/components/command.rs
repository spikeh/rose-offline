use std::time::Duration;

use bevy_ecs::prelude::Entity;
use nalgebra::{Point2, Point3};

use crate::{
    data::{item::Item, Damage, SkillId},
    game::components::{ItemSlot, MoveMode},
};

#[derive(Clone)]
pub struct CommandMove {
    pub destination: Point3<f32>,
    pub target: Option<Entity>,
    pub move_mode: Option<MoveMode>,
}

#[derive(Clone)]
pub struct CommandDie {
    pub killer: Option<Entity>,
    pub damage: Option<Damage>,
}

#[derive(Clone)]
pub struct CommandAttack {
    pub target: Entity,
}

#[derive(Clone)]
pub struct CommandPickupDroppedItem {
    pub target: Entity,
}

#[derive(Copy, Clone)]
pub enum CommandCastSkillTarget {
    Entity(Entity),
    Position(Point2<f32>),
}

#[derive(Clone)]
pub struct CommandCastSkill {
    pub skill_id: SkillId,
    pub skill_target: Option<CommandCastSkillTarget>,
    pub use_item: Option<(ItemSlot, Item)>,
}

#[derive(Clone)]
pub enum CommandData {
    Die(CommandDie),
    Stop,
    Move(CommandMove),
    Attack(CommandAttack),
    PickupDroppedItem(CommandPickupDroppedItem),
    PersonalStore,
    CastSkill(CommandCastSkill),
    // TODO:
    // Sit
}

#[derive(Clone)]
pub struct Command {
    // Current command that is executing
    pub command: CommandData,

    // How long the current command has been executing
    pub duration: Duration,

    // The duration required to complete this command, if None then the command is immediately interruptible
    pub required_duration: Option<Duration>,
}

pub struct NextCommand {
    pub command: Option<CommandData>,
    pub has_sent_server_message: bool,
}

impl NextCommand {
    pub fn default() -> Self {
        Self {
            command: None,
            has_sent_server_message: false,
        }
    }

    pub fn with_move(
        destination: Point3<f32>,
        target: Option<Entity>,
        move_mode: Option<MoveMode>,
    ) -> Self {
        Self {
            command: Some(CommandData::Move(CommandMove {
                destination,
                target,
                move_mode,
            })),
            has_sent_server_message: false,
        }
    }

    pub fn with_attack(target: Entity) -> Self {
        Self {
            command: Some(CommandData::Attack(CommandAttack { target })),
            has_sent_server_message: false,
        }
    }

    pub fn with_pickup_dropped_item(target: Entity) -> Self {
        Self {
            command: Some(CommandData::PickupDroppedItem(CommandPickupDroppedItem {
                target,
            })),
            has_sent_server_message: false,
        }
    }

    pub fn with_stop() -> Self {
        Self {
            command: Some(CommandData::Stop),
            has_sent_server_message: false,
        }
    }

    pub fn with_personal_store() -> Self {
        Self {
            command: Some(CommandData::PersonalStore),
            has_sent_server_message: false,
        }
    }

    pub fn with_cast_skill_target_self(
        skill_id: SkillId,
        use_item: Option<(ItemSlot, Item)>,
    ) -> Self {
        Self {
            command: Some(CommandData::CastSkill(CommandCastSkill {
                skill_id,
                skill_target: None,
                use_item,
            })),
            has_sent_server_message: false,
        }
    }

    pub fn with_cast_skill_target_entity(
        skill_id: SkillId,
        target_entity: Entity,
        use_item: Option<(ItemSlot, Item)>,
    ) -> Self {
        Self {
            command: Some(CommandData::CastSkill(CommandCastSkill {
                skill_id,
                skill_target: Some(CommandCastSkillTarget::Entity(target_entity)),
                use_item,
            })),
            has_sent_server_message: false,
        }
    }

    pub fn with_cast_skill_target_position(skill_id: SkillId, position: Point2<f32>) -> Self {
        Self {
            command: Some(CommandData::CastSkill(CommandCastSkill {
                skill_id,
                skill_target: Some(CommandCastSkillTarget::Position(position)),
                use_item: None,
            })),
            has_sent_server_message: false,
        }
    }
}

impl Command {
    pub fn new(command: CommandData, required_duration: Option<Duration>) -> Self {
        Self {
            command,
            duration: Duration::new(0, 0),
            required_duration,
        }
    }

    pub fn default() -> Self {
        Self::with_stop()
    }

    pub fn get_target(&self) -> Option<Entity> {
        match self.command {
            CommandData::Attack(CommandAttack { target, .. }) => Some(target),
            CommandData::Move(CommandMove { target, .. }) => target,
            _ => None,
        }
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.command, CommandData::Die(_))
    }

    pub fn with_die(
        killer: Option<Entity>,
        damage: Option<Damage>,
        duration: Option<Duration>,
    ) -> Self {
        Self::new(CommandData::Die(CommandDie { killer, damage }), duration)
    }

    pub fn with_move(
        destination: Point3<f32>,
        target: Option<Entity>,
        move_mode: Option<MoveMode>,
    ) -> Self {
        Self::new(
            CommandData::Move(CommandMove {
                destination,
                target,
                move_mode,
            }),
            None,
        )
    }

    pub fn with_attack(target: Entity, duration: Duration) -> Self {
        Self::new(
            CommandData::Attack(CommandAttack { target }),
            Some(duration),
        )
    }

    pub fn with_pickup_dropped_item(target: Entity, duration: Duration) -> Self {
        Self::new(
            CommandData::PickupDroppedItem(CommandPickupDroppedItem { target }),
            Some(duration),
        )
    }

    pub fn with_stop() -> Self {
        Self::new(CommandData::Stop, None)
    }

    pub fn with_personal_store() -> Self {
        Self::new(
            CommandData::PersonalStore,
            Some(Duration::from_secs(u64::MAX)),
        )
    }

    pub fn with_cast_skill(
        skill_id: SkillId,
        skill_target: Option<CommandCastSkillTarget>,
        casting_duration: Duration,
        action_duration: Duration,
    ) -> Self {
        Self::new(
            CommandData::CastSkill(CommandCastSkill {
                skill_id,
                skill_target,
                use_item: None,
            }),
            Some(casting_duration + action_duration),
        )
    }
}

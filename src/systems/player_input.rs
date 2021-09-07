
use crate::prelude::*;
#[system]
#[read_component(Point)]
#[read_component(Player)]

pub fn player_input( 
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    //#[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState
) 
{
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1,0),
            VirtualKeyCode::Right => Point::new(1,0),
            VirtualKeyCode::Up =>   Point::new(0,-1),
            VirtualKeyCode::Down => Point::new(0,1),
            _=> Point::new(0,0),
        }; //end of match

        players.iter(ecs).for_each(|(entity,pos)| {
            let destination = *pos + delta;
            commands
                    .push(((),WantsToMove{entity: *entity,destination}));
            }); //end of iter
                    
            *turn_state = TurnState::PlayerTurn;
    } //end of if on some

} //end of function


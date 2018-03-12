extern crate sawblade;
use self::sawblade::core::game::Game;
use self::sawblade::core::event::Event;
use self::sawblade::graphics::texture::FinalTexture;
use self::sawblade::core::world::World;
use self::sawblade::core::entity::Entity;
use self::sawblade::controllers::Controller;

fn build_world() -> Box<World> {
    Box::new(
        GameWorld::new()
    )
}

struct GameWorld {
    cubes: Vec<Cube>
}

impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            cubes: vec![]
        }
    }
}

impl World for GameWorld {
    fn init(&mut self) {
        for i in vec!(0, 100, 200) {
            for j in vec!(50, 150, 250) {
                self.cubes.push(Cube::spawn((i,j), 1));
            }
        }
    }
    fn event_loop(&mut self, events: Vec<Event>) -> Vec<FinalTexture> {
        unsafe {
            let ptr = self as *mut GameWorld;
            for cube in &mut (*ptr).cubes {
                cube.recv(ptr, "move".to_string());
            }
            let mut collected = vec![];
            for cube in &mut (*ptr).cubes {
                match cube.render() {
                    Some(texture) => collected.push(texture),
                    None => ()
                }
            }
            collected
        }
    }
}

struct MoveController {
    obj_id: u64,
    pub movement_amount: i32,
    between_counter: u32
}

impl Controller for MoveController {
    type World = GameWorld;
    fn bind(id: u64) -> MoveController {
        MoveController {
            obj_id: id,
            movement_amount: 3,
            between_counter: 0
        }
    }
    fn tick(&mut self, world: *mut GameWorld) {
        /*
        unsafe {
            (*scene).get_entity_by_id(self.obj_id).unwrap().recv(scene, "move".to_string());
        }
        */
    }
}

struct Cube {
    coordinates: (u32,u32),
    id: u64,
    movement_controller: MoveController
}

impl Entity for Cube {
    type World = GameWorld;
    fn spawn(coordinates: (u32,u32), id: u64 ) -> Cube {
        Cube {
            coordinates,
            id,
            movement_controller: MoveController::bind(id)
        }
    }
    fn get_id(&self) -> u64 {
        self.id
    }
    fn recv(&mut self, world: *mut GameWorld, trigger: String) {
        match trigger.as_str() {
            "move" => {
                self.coordinates.0 += self.movement_controller.movement_amount as u32;
            },
            _ => {}
        }
    }
    fn render(&mut self) -> Option<FinalTexture> {
        Some(FinalTexture::make_rect((50,50), self.coordinates))
    }
    fn tick(&mut self, world: *mut GameWorld) {
        self.movement_controller.tick(world);
    }
}

fn main() {
    let game = Game::new("Scene with Cube".to_string(), (500,500))
        .with_world(build_world)
        .build();
    game.start();
}
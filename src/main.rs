use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread;
use rand::Rng;
mod world;
mod mapgenerator;
mod ui;
// "technical" constants
const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;
const SCROLL_BOUNDARY_Y: f32 = 128.0;
const SCROLL_BOUNDARY_X: f32 = 256.0;
const TILE_SIZE: f32 = 32.0;
// gameplay constants
const PLAYER_SPEED: f32 = 4.0;
const PLAYER_HITBOX_SIZE: f32 = 8.0;
// controls



fn render(canvas: &mut WindowCanvas, m_x: f32, m_y: f32, attack_box: &mut world::AttackBox, player: &mut world::Entity, map: &mut world::Level,entities: &Vec<world::Entity>, camera: &mut world::Camera) {

    // per render things
    let bg_color = Color::RGB(0, 0, 0);
    
    let tile_color = Color::RGB(128,64,55);
    let floor_color = Color::RGB(64,32,30);
    let player_color = Color::RGB(128,128,0);
    canvas.set_draw_color(bg_color);
    canvas.clear();
   
      
    for x in 0..map.map_size {
	for y in 0..map.map_size {
	    // render tile
	    if map.content[y as usize][x as usize] == 1 {
		canvas.set_draw_color(tile_color);
		canvas.fill_rect(Rect::new(x as i32 * TILE_SIZE as i32 - camera.x as i32, y as i32 * TILE_SIZE as i32 - camera.y as i32, TILE_SIZE as u32, TILE_SIZE as u32));
	    }
	    
	    if map.content[y as usize][x as usize] == 0 {
		canvas.set_draw_color(bg_color);
		canvas.fill_rect(Rect::new(x as i32 * TILE_SIZE as i32 - camera.x as i32, y as i32 * TILE_SIZE as i32 - camera.y as i32, TILE_SIZE as u32, TILE_SIZE as u32));

	    }
	    
	    if map.content[y as usize][x as usize] == 2 {
		canvas.set_draw_color(floor_color);
		canvas.fill_rect(Rect::new(x as i32 * TILE_SIZE as i32 - camera.x as i32, y as i32 * TILE_SIZE as i32 - camera.y as i32, TILE_SIZE as u32, TILE_SIZE as u32));
	    }
	}
    }
    // render player
    canvas.set_draw_color(player_color);
    
    canvas.fill_rect(Rect::new(player.x as i32 - camera.x as i32, player.y as i32 - camera.y as i32, 16,16));
    // attack

    if player.attacking {
	canvas.fill_rect(Rect::new((player.x - camera.x - attack_box.x) as i32, (player.y - camera.y - attack_box.y) as i32, 16,16));
	
    }
    // render entities
    for entity in entities {
	if entity.alive {
	    canvas.fill_rect(Rect::new(entity.x as i32 - camera.x as i32, entity.y as i32 - camera.y as i32, 16,16));	
	}
    }
	canvas.present();
}


fn main_loop() -> Result<(), String> {
   //initialising windows and canvas 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Elysium", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    //initialising gameplay things
    let mut m_x = 0.0;
    let mut m_y = 0.0;
    
    let mut shoot_time = 200.0;
    let mut shoot_change = 0.0;
    let mut attack_time = 200.0;
    let mut attack_change = 0.0;
    let mut stage_completed = false;
    let mut player = world::Entity {
	
	x: 64.0,
	y: 64.0,
	speed: 4.0,
	speed_movement: 0.1,
	speed_rotation:0.1,
	hp: 3,
	ammo: 2,
	attacking:false,
	alive:true
    };
    let mut attack_box = world::AttackBox {
	x: 32.0,
	y: 32.0,
	radius: 16.0,
	angle: 0.0

    };
    let mut drill = world::Drill {
	silver: 0,
	ammo: 4,
	nitro: 1

    };
    let mut camera = world::Camera {
	x: -400.0,
	y: -300.0
    };
    let mut game_state = "neutral"; // neutral, board, intruder
    let mut worldmap = world::WorldMap {
	content: [
	    
	    [0,0, 0,0, 0,0, 0,1, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 2,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,2, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,2, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 2,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0],
	    [0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0]
	],
	    ship_x: 7,
	    ship_y: 0,
    };
    let mut map = mapgenerator::get_generated_level(0); // 0 for ship
    let mut entities = mapgenerator::get_generated_entities(0);
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut menu_on = false;
    
    let mut leftmouseclicked = false;
    let mut rightmouseclicked = false;
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
	// event handling
	for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
		// WASD
		Event::KeyDown{keycode: Some(Keycode::W), ..} => {
		    
		    w = true;
		    
		}
		Event::KeyDown{keycode: Some(Keycode::A), ..} => {
		    
		    
		    
		    a = true;
		}
		Event::KeyDown{keycode: Some(Keycode::S), ..} => {
		    
		    
		    s = true;
		}
		Event::KeyDown{keycode: Some(Keycode::D), ..} => {
		    
		    
		    d = true;
		}
		
		// WASD
		Event::KeyUp{keycode: Some(Keycode::W), ..} => {
		    
		    w = false;
		    
		}
		Event::KeyUp{keycode: Some(Keycode::A), ..} => {
		    
		    a = false;
		}
		Event::KeyUp{keycode: Some(Keycode::S), ..} => {
		    
		    
		    s = false;
		}
		Event::KeyUp{keycode: Some(Keycode::D), ..} => {
		    
		    
		    d = false;
		}

		Event::KeyUp{keycode: Some(Keycode::M), ..} => {
		    
		    
		    menu_on = !menu_on;
		}

		// Mouse
		
                _ => {}
            }
        }
	 if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left)
        {
	    
	    if !leftmouseclicked {
		let mut state = event_pump.mouse_state();
		
		if !player.attacking {
		    m_x = state.x() as f32;
		    m_y = state.y() as f32;
		    player.attacking = true;
		}
		
		leftmouseclicked = true;
	    }
	}
	else {
	    leftmouseclicked = false;
	}

	    if !rightmouseclicked {
		let mut state = event_pump.mouse_state();
		
		
		rightmouseclicked = true;
	    }
	else {
	    rightmouseclicked = false;
	}

	if w == true {
	    if map.content[((player.y - PLAYER_SPEED) / TILE_SIZE) as usize][((player.x) / TILE_SIZE) as usize] != 1 {
		player.y -= PLAYER_SPEED;
	    }
	    if player.y < camera.y + SCROLL_BOUNDARY_Y {
		camera.y -= PLAYER_SPEED;
	    }
	}
	
	if a == true {

	    
	    if map.content[((player.y) / TILE_SIZE) as usize][((player.x - PLAYER_SPEED) / TILE_SIZE) as usize] != 1 {
		player.x -= PLAYER_SPEED;
	    }
	    if player.x < camera.x + SCROLL_BOUNDARY_X {
		camera.x -= PLAYER_SPEED;
	    }
	}
	
	if s == true {

	    
	    if map.content[((player.y + PLAYER_SPEED*4.0) / TILE_SIZE) as usize][((player.x) / TILE_SIZE) as usize] != 1 {
		player.y += PLAYER_SPEED;
	    }
	    if player.y > camera.y + SCREEN_HEIGHT as f32 - SCROLL_BOUNDARY_Y {
		camera.y += PLAYER_SPEED;
	    }
	}
	
	if d == true {

	    if map.content[((player.y) / TILE_SIZE) as usize][((player.x + PLAYER_SPEED*4.0) / TILE_SIZE) as usize] != 1 {
		player.x += PLAYER_SPEED;
	    }
	    
	    if player.x > camera.x + SCREEN_WIDTH as f32 - SCROLL_BOUNDARY_X {
		camera.x += PLAYER_SPEED;
	    }
	}
        
    // ui (command line ui is in its separate thread)
	while menu_on == true {
	    ui::draw_ui(game_state, &player);
	    
	    let mut line = String::new();
	    
	    let input = std::io::stdin().read_line(&mut line).unwrap();
	    if line.trim() == "q" {
		menu_on = false;
	    }
	    if line.trim() == "intruder" {
		game_state = "intruder";
		println!("Intruder alert!");
		map = mapgenerator::get_generated_level(0);
		player.reset();
		
	    }
	    if line.trim() == "board" {
		game_state = "board";
		map = mapgenerator::get_generated_level(1); // 0 for ship
		entities = mapgenerator::get_generated_entities(1);
		player.reset();
	    }
	    if line.trim() == "drill" {
		let scenario = ui::draw_descend(&mut worldmap);
		if scenario.title.trim() == "loot" {
		    let mut rng = rand::thread_rng();
		    drill.find_loot();
		}
		else if scenario.title.trim() == "board" {
		    game_state = "board";
		    println!("You prepare to board...");
		    
		    map = mapgenerator::get_generated_level(1); // 0 for ship
		    entities = mapgenerator::get_generated_entities(1);
		    player.reset();
		}
		
		else if scenario.title.trim() == "intruder" {
		    println!("Intruder alert!");
		}
		else if scenario.title.trim() == "neutral" {
		    
		    println!("You continue drilling...");
		}
	    }
	    else {
	    }
	}
   

	// logic

	
	if player.attacking {
	    attack_box.angle = (player.y - camera.y - m_y).atan2(player.x - camera.x - m_x);
	    attack_box.x = attack_box.angle.cos() * 16.0;
	    attack_box.y = attack_box.angle.sin() * 16.0;
	}
	for mut entity in &mut entities {
	    // entity logic, put in separate function!
	    
	    let dist_to_player = ((player.x - entity.x).powf(2.0) + (player.y - entity.y).powf(2.0)).sqrt();
	    let angle_to_player = (player.y - entity.y).atan2(player.x - entity.x);
	    
	    if dist_to_player < PLAYER_HITBOX_SIZE {
		player.hp -= 1;
		
	    }
	    if player.attacking {
		
		let dist_from_attack = ((entity.x - player.x+ attack_box.x).powf(2.0) + (entity.y -player.y+ attack_box.y).powf(2.0)).sqrt();
		if dist_from_attack < attack_box.radius {
		    entity.hp -= 1;
		}
	    }
	    // movement 
	    entity.x += angle_to_player.cos() * entity.speed_movement;
	    entity.y += angle_to_player.sin() * entity.speed_movement;
	    
	    // death
	    
	    if entity.hp < 1 {
		entity.alive = false;
		stage_completed = true;
		
	    }
	}
	if stage_completed {

	    game_state = "neutral";
	    map = mapgenerator::get_generated_level(0); // 0 for ship
	    entities = mapgenerator::get_generated_entities(0);
	    player.reset();
	    stage_completed = false;
	}
	// render
        render(&mut canvas, m_x, m_y, &mut attack_box, &mut player, &mut map, &entities, &mut camera);
	let duration = Duration::new(0, 1_000_000_000u32 / 60);

	// counters
	if player.attacking {
	    attack_change += duration.as_millis() as f32;
	}

	if attack_change > attack_time {
	    player.attacking = false;
	    attack_change = 0.0;
	}
	// sleep
	::std::thread::sleep(duration);
    }
    Ok(())
}

fn main() {

main_loop();
}

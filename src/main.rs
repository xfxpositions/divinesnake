
use math::round::floor;
pub use macroquad::prelude::*  ;

use amethyst_core::{frame_limiter, math::ComplexField, num::Float};
use macroquad::rand::gen_range;
use ::rand::{Rng, thread_rng};



#[macroquad::main("divine snake")]
async fn main() {
    
    let mut fps_limiter = frame_limiter::FrameLimiter::new(frame_limiter::FrameRateLimitStrategy::Sleep,20);

    let mut part_size:f32 = 20f32;
    const speed:f32 = 10f32;
    struct Gamearea{
        width:f32,
        height:f32,
        color:Color,
        paddingX:f32,
        paddingY:f32
    }
    impl Gamearea{
        pub fn init(self:&mut Self){
            let width = screen_width();
            let height = screen_height();
            let d = floor(width as f64,-2) as f32;
            let f = floor(height as f64,-2) as f32;
            self.width = d;
            self.height = f;
        }
    }
    struct Trail{
        pX:f32,
        pY:f32
    }
    struct Apple{
        pX:f32,
        pY:f32
    }
    impl Apple {
        pub fn init(self:&mut Self,gamearea:&Gamearea){
            let mut width = screen_width();
            let mut height = screen_height();
            let mut rng = thread_rng();
            let mut pX:f32 = (rng.gen_range(0..gamearea.width as u32)/10) as f32;
            let mut pY:f32 = (rng.gen_range(0..gamearea.height as u32)/10) as f32;
            pX.ceil();
            pY.ceil();
            pX*=10f32;
            pY*=10f32;
            self.pX = pX;
            self.pY = pY;
            println!("appleX {}",pX);
        }
    }
    struct Snake<'a>{
        pX:f32,
        pY:f32,
        velocityX:f32,
        velocityY:f32,
        trails:&'a mut Vec<Trail>,
        trail_lenght:usize
    }
    let mut apple:Apple = Apple { pX: 0f32, pY: 0f32 };
    let mut gamearea:Gamearea = Gamearea { width: 0f32, height:0f32, color: WHITE,paddingX:20f32,paddingY:20f32};
    gamearea.init(); //needs to be initilaze first!
    apple.init(&gamearea); //needs to be initilaze first!
    println!("appleX is {}, appleY is {}",apple.pX,apple.pY);

    let mut trails:Vec<Trail> = vec![];
    

    let mut snake:Snake = Snake{pX:300f32,pY:300f32,velocityX:1f32,velocityY:0f32,trails:&mut trails,trail_lenght:3 as usize};
    
    fn snake_move(snake:&mut Snake,apple:&mut Apple,gamearea:&Gamearea){
        if(snake.pX == 0f32){
            snake.pX = gamearea.width + gamearea.paddingX;
        }
        else if(snake.pX == gamearea.width){
            snake.pX = gamearea.paddingX + 1f32;
        }
        else if(snake.pY == 0f32){
            snake.pY = gamearea.height + gamearea.paddingY;
        }
        else if(snake.pY == gamearea.height){
            snake.pY = gamearea.paddingY + 1f32 ;
        }
        //move head
        snake.pX += snake.velocityX*speed;
        snake.pY += snake.velocityY*speed;
        if(snake.pX == apple.pX && snake.pY == apple.pY){
            snake.trail_lenght += 1;
            apple.init(gamearea);
        }
       
        //move trails
        let trail_to_push = Trail{pX:snake.pX,pY:snake.pY};
        
        if(snake.trails.len() > snake.trail_lenght){
            snake.trails.remove(0);
            snake.trails.push(trail_to_push);
            println!("{}",snake.trail_lenght);
        }
        else{
            snake.trails.push(trail_to_push);
        }
        
    }
    fn snake_draw(snake:&mut Snake,part_size:f32){
        draw_rectangle(snake.pX, snake.pY, 20f32, 20f32, WHITE); //draw head
        for trail in snake.trails.iter(){
            draw_rectangle(trail.pX, trail.pY, part_size-2f32, part_size-2f32, GREEN); //draw trails
        }

    }
    fn draw_apple(apple:&Apple){
        draw_rectangle(apple.pX, apple.pY, 20f32, 20f32, PINK); //draw apple
    }
    fn limit_fps(fps_limiter:&mut frame_limiter::FrameLimiter){
        let fps = macroquad::time::get_fps().to_string();
        fps_limiter.start();
        fps_limiter.wait();
        draw_text(&fps,20f32,20f32,40f32,WHITE);
    }
    fn update_direction(snake:&mut Snake){
        // println!("{}",is_key_down(KeyCode::Up));
        if(is_key_down(KeyCode::Up) && snake.velocityY != 1f32){
            snake.velocityX = 0f32;
            snake.velocityY = -1f32;
        }
        if(is_key_down(KeyCode::Down) && snake.velocityY != -1f32){
            snake.velocityX = 0f32;
            snake.velocityY = 1f32;
        }
        if(is_key_down(KeyCode::Left) && snake.velocityX != 1f32){
            snake.velocityX = -1f32;
            snake.velocityY = 0f32;
        }
        if(is_key_down(KeyCode::Right) && snake.velocityX != -1f32){
            snake.velocityX = 1f32;
            snake.velocityY = 0f32;
        }
        if(is_key_down(KeyCode::Space)){
            snake.velocityX = 0f32;
            snake.velocityY = 0f32;
        }
        //println!("velocityX is =>{}, velocityY is => {}",snake.velocityX,snake.velocityY);   
             
    }
    fn set_screen(mut width:f32,mut height:f32, mut part_size:f32){
        width = screen_width();
        height = screen_height();
        part_size = width /300f32;
        println!("part_size is =>{}",part_size);
    }
    fn draw_gamearea(gamearea:&mut Gamearea){
        gamearea.init();
        draw_rectangle_lines(gamearea.paddingX, gamearea.paddingY, gamearea.width, gamearea.height, 3f32, WHITE);
    }
    loop{
        clear_background(BLACK);
        draw_gamearea(&mut gamearea);
        //println!("snake_px: {} snake_pY: {}", snake.pX,snake.pY);
        snake_draw(&mut snake,part_size);
        draw_apple(&apple);
        println!("{},{}",snake.pX,snake.pY);
        update_direction(&mut snake);
        snake_move(&mut snake,&mut apple,&gamearea);
        limit_fps(&mut fps_limiter);        
        next_frame().await;
    }
}
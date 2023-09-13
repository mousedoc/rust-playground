
// derive - 파생
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: u32,
}

// Implementaion
impl Rectangle {
    // &self --- Only read instance 
    fn size(&self) -> u32 {
        self.width * self.height
    }

    // &mut self --- Modify instance
    fn reverse(&mut self) {
        let temp= self.width;
        self.width = self.height;
        self.height = temp;
    }

    // self --- Consume instance
    fn convert_circle_by_width(self) -> Circle {
        let circle: Circle =  Circle {
            radius: self.width,
        };

        circle
    }
}

// can impl multiple times
impl Rectangle {
    // like static method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    basic_rectangle();
    implementaion_rectangle();
}

fn basic_rectangle() {
    let rect = Rectangle {
        width: 24,
        height: 8,
    };

    // rect is 192
    println!("size is {}", rect.size());

    // {:?} dispaly for debug
    // rect is Rectangle { width: 24, height: 8 }
    println!("rect is {:?}", rect);         

    // {:#?} dispaly more pretty for debug
    // rect is Rectangle {
    //     width: 24,
    //     height: 8,
    // }
    println!("rect is {:#?}", rect);

    // [src\main.rs:33] rect = Rectangle {
    //     width: 24,
    //     height: 8,
    // }
    dbg!(&rect);
    dbg!(rect);

    // cannot cuz dbg!(...) takes ownership so, you should be using with ref
    // println!("dbg takes ownership {}", rect.width);

    // [src\main.rs:45] 3 * scale = 6
    let scale = 2;
    let new_rect = Rectangle {
        width: dbg!(3 * scale),
        height: 12,
    };

    dbg!(&new_rect);
}

fn implementaion_rectangle() {
    let mut rect: Rectangle = Rectangle {
        width: 12,
        height: 2,
    };

    let size = rect.size();
    println!("rect size: {}", size);

    rect.reverse();
    println!("rect width/height: {}/{}", rect.width, rect.height);

    let circle = rect.convert_circle_by_width();
    println!("radius: {}", circle.radius);

    // cannot, cuz convert_circle_by_width() consumed rect
    // println!("rect width/height: {}/{}", rect.width, rect.height);

    let square = Rectangle::square(32);
    println!("square size: {}", square.size());
}
use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Hiboy S2 MAX Electric Scooter".to_string(),
            price: 637.99,
            description: "500W Motor/ 65km Range/ 31km/h / Pneumatic Tires/ Dual Braking System, Portable and Foldable Commuter E-scooter for Adults (Grey Color)".to_string(),
            image: "/HiBoy-S2.jpeg".to_string()
        },
        Product {
            id: 2,
            name: "Shokz OpenRun Pro".to_string(),
            price: 139.99,
            description: " Bone Conduction Open-Ear Bluetooth Headphones - Steel Blue - Only at Best Buy.".to_string(),
            image: "/Shokz.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5".to_string(),
            price: 12.99,
            description: "Over-Ear Noise Cancelling Bluetooth Headphones - Black - Open Box.".to_string(),
            image: "/WH1000.jpeg".to_string()
        },
        Product {
            id: 4,
            name: "ASUS ROG Strix G13 Gaming PC".to_string(),
            price: 1199.99,
            description: "Dark Grey (Intel Core i5-14400F/1TB SSD/16GB RAM/RTX 4060/Win11) - Only at Best Buy.".to_string(),
            image: "/ASUS.jpg".to_string()
        },
        Product {
            id: 5,
            name: "DeLonghi Magnifica Evo Automatic Espresso Maker".to_string(),
            price: 999.99,
            description: "With Frother & Grinder & Over Ice Function-Silver/Black.".to_string(),
            image: "/DeLonghi.jpg".to_string()
        },
        Product {
            id: 6,
            name: "MotionGrey Standing Desk".to_string(),
            price: 109.99,
            description: "Height Adjustable Electric Motor Sit-to-Stand Desk Computer for Home and Office - White Frame (43x24 White Tabletop Included) - Only on Best Buy.".to_string(),
            image: "/Desk.jpeg".to_string()
        },
        Product {
            id: 7,
            name: "WINGOMART 3-Piece Luggage Set".to_string(),
            price: 199.99,
            description: "Lightweight Durable PC+ABS Hardshell, Double Spinner Wheels, TSA Lock - 20in/24IN/28in - Gold.".to_string(),
            image: "/WINGOMART.jpeg".to_string()
        },
        Product {
            id: 8,
            name: "Digital Camera 4K 48MP Vlogging Camera".to_string(),
            price: 299.99,
            description: "Cameras for Photography and YouTube, 16X Digital Zoom, with 180° Flip Screen,2 Batteries, 32GB TF Card.".to_string(),
            image: "/Camera.jpeg".to_string()
        },
        Product {
            id: 9,
            name: "Turtle Beach Vulcan II".to_string(),
            price: 89.99,
            description: "Max Backlit Mechanical Optical Red Switches Full-Size Gaming Keyboard - Only at Best Buy.".to_string(),
            image: "/TurtleBeach.jpg".to_string()
        }
    ]
}
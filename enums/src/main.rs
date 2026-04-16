enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Usiamo match per gestire ogni caso possibile
        match self {
            Message::Quit => {
                println!("Il programma sta uscendo...");
            }
            Message::Move { x, y } => {
                println!("Spostamento alle coordinate x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("Messaggio ricevuto: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Cambio colore a R: {}, G: {}, B: {}", r, g, b);
            }
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    // definisci un enum Message
    let c1 = 11;
    let c2 = 22;
    let c3 = 33;
    let x = 0;
    let y = 0;
    let msg1 = Message::Write(String::from("Hello, World!"));
    let msg2 = Message::ChangeColor(c1, c2, c3);
    let msg3 = Message::Move { x, y };
    let msg4 = Message::Quit;
    // chiama il metodo Message.call()
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
}

enum OrderStatus {
    Pending,                              // 접수 대기
    Confirmed { order_id: u32 },          // 확정 - 주문번호 부여됨
    Delivering { driver: String },        // 배송 중 - 기사 이름
    Delivered { rating: u8 },             // 완료 - 별점
    Cancelled { reason: String },         // 취소 - 이유
}

fn describe(status: &OrderStatus) {
    match status {
        OrderStatus::Pending =>
            println!("접수 대기 중"),
        OrderStatus::Confirmed { order_id } =>
            println!("주문 {} 확정됨", order_id),
        OrderStatus::Delivering { driver } =>
            println!("{} 기사님이 배송 중", driver),
        OrderStatus::Delivered { rating } =>
            println!("완료 (별 {}개)", rating),
        OrderStatus::Cancelled { reason } =>
            println!("취소: {}", reason),
    }
}

fn main() {
    let orders = [
        OrderStatus::Pending,
        OrderStatus::Confirmed { order_id: 1234 },
        OrderStatus::Delivering { driver: String::from("김철수") },
        OrderStatus::Delivered { rating: 5 },
        OrderStatus::Cancelled { reason: String::from("품절") },
    ];

    for o in &orders {
        describe(o);
    }
}

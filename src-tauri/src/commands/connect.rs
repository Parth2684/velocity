

enum UserType {
    Sender,
    Receiver
}


pub async fn connect(user_type: UserType) {
    match user_type {
        UserType::Sender => todo!(),
        UserType::Receiver => todo!()
    }
}
mod remoc {
    use remoc::rch::mpsc::{self, channel};

    pub type Sender<T> = mpsc::Sender<T, remoc::codec::Bincode>;
    pub type Receiver<T> = mpsc::Receiver<T, remoc::codec::Bincode>;

    async fn send_task(sender: Sender<()>) {
        for _ in 0..16 {
            sender.send(()).await.unwrap();
        }
    }

    async fn receive_task(receiver: Receiver<()>) {
        let distributor = receiver.distribute(false);
        for _ in 0..16 {
            let (mut receiver, handle) = distributor.subscribe().await.unwrap();
            receiver.recv().await.unwrap().unwrap();
            dbg!("Received value");
            handle.remove();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn lost_messages() {
            let (sender, receiver) = channel(64);
            tokio::join!(send_task(sender), receive_task(receiver));
        }
    }
}

mod async_channel {
    use async_channel::{Receiver, Sender};

    async fn send_task(sender: Sender<()>) {
        for _ in 0..16 {
            sender.send(()).await.unwrap();
        }
    }

    async fn receive_task(receiver: Receiver<()>) {
        for _ in 0..16 {
            let receiver_clone = receiver.clone();
            receiver_clone.recv().await.unwrap();
            dbg!("Received value");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn no_lost_messages() {
            let (sender, receiver) = async_channel::bounded(64);
            tokio::join!(send_task(sender), receive_task(receiver));
        }
    }
}

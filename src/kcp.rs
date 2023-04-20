use std::collections::LinkedList;

use crate::Message;

pub struct Kcp {
    // 应用层调用 send，把消息写入 snd_queue
    snd_queue: LinkedList<u8>, // TOUP 手写循环双链表
    // 协议层时钟到了，调用 update -> flush，把 snd_queue 的消息写到 snd_buff
    snd_buf: LinkedList<u8>,
    // 应用层调用 recv，从 recv_queue 获取消息
    recv_queue: LinkedList<u8>,
    // 协议层收到传输层的消息，把消息写入 recv_buf，然后再写入 recv_queue
    recv_buf: LinkedList<u8>,
}

impl Kcp {
    /// input 的输入和输出声明是什么？
    /// 输入可以由 UDP 给到，那更一般的场景是什么？
    /// `办法1`：
    /// 由协议负责消息的解析和分界
    pub fn input(&mut self, buf: &mut [u8]) { // TOUP 用 ByteMut 替换字节数组
        // 数据流里可能有零个或多个消息，要逐个解析
        let mut start: usize = 0; // 注意，下标从0开始
        while let len = Message::check(&buf[start..]) {
            if len == 0 {
                break;
            }

            match Message::parse(&buf[start..]) {
                Ok(message) => match message {
                    Message::Ack(header) => {

                    }
                    Message::Psh(header, data) => {
    
                    }
                    Message::Wack(header) => {
    
                    }
                    Message::Wins(header) => {
    
                    }
                }
                Err(error) => {
                    // 解析出错后，后面的数据暂时没有修复的能力

                }
            }

            start += len;
        }
    }
}
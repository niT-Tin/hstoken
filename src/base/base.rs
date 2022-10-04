use std::collections::LinkedList;
use std::slice::Iter;

pub struct PPUTKIterator<'a, char> {
    // peek缓存
    // 用于和putback缓存进行交互，
    // 实现在putback中的从next缓存中取元素放回putback缓存
    // 而不用实际调用next方法
    peekc: LinkedList<&'a char>,
    // putback缓存
    putbk: LinkedList<&'a char>,
    // 缓存大小
    buffer_size: usize,
    // 字符流
    stream: Iter<'a, char>,
}

impl<'a, char> PPUTKIterator<'a, char> {
    pub fn new(iter: Iter<'a, char>) -> Self {
        let pkc = LinkedList::new();
        let pbk = LinkedList::new();
        PPUTKIterator {
            peekc: pkc,
            putbk: pbk,
            buffer_size: 10,
            stream: iter,
        }
    }

    // 将上一次通过next从流中获得的数据放回流中
    pub fn put_back(&mut self) {
        if self.peekc.len() != 0 {
            let pk = self.peekc.pop_back();
            match pk {
                Some(p) => self.putbk.push_back(p),
                None => {},
            }
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        let res = self.next();
        match res {
            Some(s) => {
                self.put_back();
                Some(s)
            },
            None => None,
        }
    }
}

impl<'a, char> Iterator for PPUTKIterator<'a, char> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        // 如果putback的缓存不为空，则再次调用next的时候返回putback缓存中的数据
        if self.putbk.len() != 0usize {
            // 加入peek缓存，以备putback使用
            let res = self.putbk.pop_back();
            if let Some(r) = res {
                // 此处再次放回peek缓存是因为可能存在再次putback的情况
                self.peekc.push_back(r);
            }
            return res;
        }
        // 获取下一个元素不可变引用
        let next = self.stream.next();
        // 如果putback缓存大于限定缓存大小，弹出最开始的元素
        if self.peekc.len() > self.buffer_size {
            self.peekc.pop_front();
        }
        match next {
            Some(t) => {
                // 加入peek缓存，以备putback使用
                self.peekc.push_back(t);
                Some(t)
            }
            None => None,
        }
    }

}

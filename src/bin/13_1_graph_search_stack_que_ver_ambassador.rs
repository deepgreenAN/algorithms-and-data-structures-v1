use std::collections::VecDeque;

use ambassador::{delegatable_trait, Delegate};

#[derive(Debug)]
struct Stack<T> {
    inner_vec: Vec<T>,
}

#[derive(Debug)]
struct Queue<T> {
    inner_vec_deque: VecDeque<T>,
}

#[delegatable_trait]
pub trait StackLike {
    type Item;
    fn push(&mut self, x: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
}

impl<T> Stack<T> {
    fn new(new_vec: Vec<T>) -> Self {
        let mut instance = Self {
            inner_vec: Vec::new(),
        };

        for item in new_vec.into_iter() {
            instance.inner_vec.push(item);
        }
        instance
    }
}

impl<T> StackLike for Stack<T> {
    type Item = T;
    fn push(&mut self, x: T) {
        self.inner_vec.push(x);
    }
    fn pop(&mut self) -> Option<T> {
        self.inner_vec.pop()
    }
    fn is_empty(&self) -> bool {
        self.inner_vec.is_empty()
    }
}

impl<T> Queue<T> {
    fn new(new_vec: Vec<T>) -> Self {
        let mut instance = Self {
            inner_vec_deque: VecDeque::new(),
        };

        for item in new_vec.into_iter() {
            instance.inner_vec_deque.push_back(item);
        }
        instance
    }
}

impl<T> StackLike for Queue<T> {
    type Item = T;
    fn push(&mut self, x: T) {
        self.inner_vec_deque.push_back(x);
    }
    fn pop(&mut self) -> Option<T> {
        self.inner_vec_deque.pop_front()
    }
    fn is_empty(&self) -> bool {
        self.inner_vec_deque.is_empty()
    }
}

/// 深さ優先か幅優先かを示す列挙体
enum SearchMethod {
    Depth,
    Breadth,
}

/// Todoリストを表す列挙体
#[derive(Debug, Delegate)]
#[delegate(StackLike)]
enum TodoList<T> {
    Stack(Stack<T>),
    Queue(Queue<T>),
}

/// TodoListのコンストラクタのみ定義
impl<T> TodoList<T> {
    fn new(new_vec: Vec<T>, search_method: SearchMethod) -> Self {
        match search_method {
            SearchMethod::Depth => Self::Stack(Stack::new(new_vec)),
            SearchMethod::Breadth => Self::Queue(Queue::new(new_vec)),
        }
    }
}

type Graph = Vec<Vec<usize>>; // グラフの型エイリアス

fn graph_search(g: &Graph, s: usize, search_method: SearchMethod) -> Vec<usize> {
    let n = g.len(); // グラフの頂点数

    let mut seen = vec![false; n]; // 訪問したかどうかを示す配列
    let mut todo = TodoList::new(vec![], search_method);

    // 初期条件
    seen[s] = true; // s は探索済みとする
    todo.push(s); // todoはsのみを含む

    let mut path: Vec<usize> = Vec::new();

    while !todo.is_empty() {
        // todoが空になるまで探索を行う
        // todoからpopする
        let v: usize = todo.pop().unwrap(); // 探索する頂点
        path.push(v);

        // vからたどれる頂点を全て調べる
        for x in g[v].iter() {
            if seen[*x] {
                // すでに発見済みの頂点は探索しない
                continue;
            }

            // 新たな頂点xを探索済みとしてtodoに挿入
            seen[*x] = true;
            todo.push(*x);
        }
    }

    path
}

fn main() {
    let g: Graph = vec![
        vec![1, 2, 4],
        vec![0, 3, 8],
        vec![0, 5],
        vec![1, 7, 8],
        vec![0, 8],
        vec![2, 6, 8],
        vec![5, 7],
        vec![3, 6],
        vec![1, 3, 4],
    ];

    let path = graph_search(&g, 0_usize, SearchMethod::Depth);
    println!("depth first search path:{:?}", path);

    let path = graph_search(&g, 0_usize, SearchMethod::Breadth);
    println!("breadth first search path:{:?}", path);
}

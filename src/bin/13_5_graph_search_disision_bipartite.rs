type Graph = Vec<Vec<usize>>;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Color {
    White,
    Black,
    None,
}

fn change_color(color: Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
        Color::None => Color::None,
    }
}

fn dfs(g: &Graph, v: usize, color_list: &mut [Color], one_color: Color) -> bool {
    color_list[v] = one_color;
    for x in g[v].iter() {
        // 隣接頂点がすでに色確定していた場合
        if color_list[*x] != Color::None {
            // 同じ色が隣接した場合は二部グラフではない
            if color_list[*x] == one_color {
                return false;
            }

            // 色が確定した場合には探索しない
            continue;
        }

        // 隣接頂点の色を変えて、再帰的に探索(falseが返ってきたらfalseを返す)
        if !dfs(g, *x, color_list, change_color(one_color)) {
            return false;
        }
    }

    true
}

fn main() {
    let g = vec![vec![1, 3], vec![2, 0], vec![1], vec![0, 4], vec![1, 3]];

    let mut color_list = vec![Color::None; g.len()];
    let mut is_bipartite = true;

    for v in 0..g.len() {
        // vが探索済みの場合は探索しない
        if color_list[v] != Color::None {
            continue;
        }
        if !dfs(&g, v, &mut color_list, Color::White) {
            is_bipartite = false;
        } // 一つでもfalseならfalse
    }

    if is_bipartite {
        println!("This is biparite. graph color:{:?}", color_list);
    } else {
        println!("This is not bipartite.");
    }
}

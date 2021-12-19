use crate::dbg;
use crate::graph::compressed_graph::CompressedGraph;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::edges::simple_edge::SimpleEdge;
use crate::graph::graph_trait::GraphTrait;
use std::time::Instant;

struct State {
    v: u32,
    g_pos: u32,
}

fn dfs1<G>(v: usize, stack: &mut Vec<State>, used: &mut [bool], g: &G, order: &mut Vec<u32>)
where
    G: GraphTrait<SimpleEdge>,
{
    stack.push(State {
        v: v as u32,
        g_pos: 0,
    });
    used[v] = true;
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        let edges = g.adj(cur_state.v as usize);
        loop {
            match edges.get(cur_state.g_pos as usize) {
                None => {
                    order.push(cur_state.v);
                    stack.pop();
                    break;
                }
                Some(edge) => {
                    let next = edge.to();
                    if used[next] {
                        cur_state.g_pos += 1;
                        continue;
                    }
                    used[next] = true;
                    stack.push(State {
                        v: next as u32,
                        g_pos: 0,
                    });
                    break;
                }
            }
        }
    }
}

fn dfs2<G>(
    v: usize,
    stack: &mut Vec<State>,
    used: &mut [bool],
    g_rev: &G,
    cur_comp_id: u32,
    comp_id: &mut [u32],
) where
    G: GraphTrait<SimpleEdge>,
{
    stack.push(State {
        v: v as u32,
        g_pos: 0,
    });
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        let adj = g_rev.adj(cur_state.v as usize);
        match adj.get(cur_state.g_pos as usize) {
            None => {
                comp_id[cur_state.v as usize] = cur_comp_id;
                stack.pop();
                continue;
            }
            Some(edge) => {
                let next = edge.to();
                if used[next] {
                    cur_state.g_pos += 1;
                    continue;
                }
                used[next] = true;
                stack.push(State {
                    v: next as u32,
                    g_pos: 0,
                });
            }
        }
    }
}

fn rev_graph<G>(graph: &G) -> impl GraphTrait<SimpleEdge>
where
    G: GraphTrait<SimpleEdge>,
{
    let iter = || {
        (0..graph.num_vertices()).flat_map(move |v| {
            graph
                .adj(v)
                .iter()
                .map(move |edge| (edge.to(), SimpleEdge::new(v)))
        })
    };
    CompressedGraph::with_edge_iter(graph.num_vertices(), iter())
}

// TODO: usize -> u32
pub fn find_strongly_connected_component<G>(graph: &G) -> Vec<u32>
where
    G: GraphTrait<SimpleEdge>,
{
    let n = graph.num_vertices();
    let mut comp_id = vec![0; n];
    let mut used = vec![false; n];
    let n = graph.num_vertices();
    let mut stack = vec![];

    let mut order: Vec<u32> = Vec::with_capacity(n);
    {
        let before = Instant::now();
        for v in 0..n {
            if !used[v] {
                dfs1(v, &mut stack, &mut used, graph, &mut order);
            }
        }
        dbg!("dfs1", before.elapsed().as_millis());
    }

    let mut used = vec![false; n];
    let mut cur_comp_id = 0;
    {
        let before = Instant::now();
        let rev_graph = rev_graph(graph);
        dbg!("rev graph", before.elapsed().as_millis());

        let before = Instant::now();
        for &v in order.iter().rev() {
            if used[v as usize] {
                continue;
            }
            dfs2(
                v as usize,
                &mut stack,
                &mut used,
                &rev_graph,
                cur_comp_id,
                &mut comp_id,
            );
            cur_comp_id += 1;
        }
        dbg!("dfs2", before.elapsed().as_millis());
    }
    comp_id
}

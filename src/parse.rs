use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, not_line_ending, space0},
    combinator::opt,
    sequence::delimited,
};
use rustc_hash::FxHashMap;
use std::str::FromStr;

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedNode {
    id: String,
    desc: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedEdge {
    src: String,
    dest: String,
    directed: bool,
    desc: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Graph {
    nodes: Vec<String>,
    edges: Vec<ParsedEdge>,
}

#[allow(dead_code)]
pub enum ParsedLine {
    Node(ParsedNode),
    Edge(ParsedEdge),
    Blank,
    Comment(String),
    Error,
}

// Parse identifier: alphanumeric and underscore allowed
fn parse_id(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

// Parse node line: (id[node text])
fn parse_node(input: &str) -> IResult<&str, ParsedNode> {
    let (input, id) = parse_id(input)?;
    let (input, desc) = delimited(tag("["), take_until("]"), tag("]")).parse(input)?;
    Ok((
        input,
        ParsedNode {
            id: id.to_string(),
            desc: desc.to_string(),
        },
    ))
}

// Parse optional edge description: |edge|
fn parse_edge_desc(input: &str) -> IResult<&str, &str> {
    delimited(tag("|"), take_until("|"), tag("|")).parse(input)
}

// Parse edge line: (id1 --> |desc| id2) or (id1 <--> |desc| id2)
// Edge description is optional
fn parse_edge(input: &str) -> IResult<&str, ParsedEdge> {
    let (input, src) = parse_id(input)?;
    let (input, _) = space0(input)?;
    let (input, dir) = alt((tag("-->"), tag("<-->"))).parse(input)?;
    let directed = dir == "-->";
    let (input, _) = space0(input)?;
    let (input, desc) = opt(parse_edge_desc).parse(input)?;
    let (input, _) = space0(input)?;
    let (input, dest) = parse_id(input)?;
    Ok((
        input,
        ParsedEdge {
            src: src.to_string(),
            dest: dest.to_string(),
            directed,
            desc: desc.unwrap_or("").to_string(),
        },
    ))
}

// Parse comment lines starting with '%%', ignores content
fn parse_comment(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("%%")(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, input.to_string()))
}

// Parse a single line as either node, edge, comment, or empty line
fn parse_line(input: &str) -> IResult<&str, ParsedLine> {
    let (input, _) = space0(input)?;

    // Try parse empty
    if input.is_empty() {
        return Ok((input, ParsedLine::Blank));
    }
    // Try parse comment
    if let Ok((input, comment)) = parse_comment(input) {
        return Ok((input, ParsedLine::Comment(comment)));
    }
    // Try parse node
    if let Ok((input, node)) = parse_node(input) {
        return Ok((input, ParsedLine::Node(node)));
    }
    // Try parse edge
    if let Ok((input, edge)) = parse_edge(input) {
        return Ok((input, ParsedLine::Edge(edge)));
    }
    // If line is blank or cannot parse, skip
    Ok((input, ParsedLine::Error))
}

// Parse the entire input text into graph with nodes and edges
fn parse_graph(input: &str) -> Result<Graph, String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_map = FxHashMap::default();
    for line in input.lines() {
        match parse_line(line) {
            Ok((input, result)) => match result {
                ParsedLine::Node(node) => {
                    if let Some(id) = node_map.get(&node.id) {
                        let id = *id;
                        nodes[id] = node.id.clone();
                    } else {
                        node_map.insert(node.id.clone(), nodes.len());
                    }
                    nodes.push(node.desc);
                }
                ParsedLine::Edge(edge) => {
                    if !node_map.contains_key(&edge.src) {
                        node_map.insert(edge.src.clone(), nodes.len());
                    }
                    if !node_map.contains_key(&edge.dest) {
                        node_map.insert(edge.dest.clone(), nodes.len());
                    }
                    edges.push(edge)
                }
                ParsedLine::Blank | ParsedLine::Comment(_) => (),
                ParsedLine::Error => {
                    return Err(format!("Failed to parse line '{}'", input));
                }
            },
            Err(e) => return Err(format!("Failed to parse line '{}': {:?}", line, e)),
        }
    }
    Ok(Graph { nodes, edges })
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_graph(s)
    }
}

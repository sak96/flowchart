use nom::{
    Err, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{not_line_ending, space0},
    combinator::opt,
    sequence::delimited,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Node {
    id: String,
    desc: String,
}

#[derive(Debug)]
pub struct Edge {
    src: String,
    dest: String,
    directed: bool,
    desc: String,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

pub enum ParsedLine {
    Node(Node),
    Edge(Edge),
    Blank,
    Comment(String),
    Error,
}

// Parse identifier: alphanumeric and underscore allowed
fn parse_id(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

// Parse node line: (id[node text])
fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, id) = parse_id(input)?;
    let (input, desc) = delimited(tag("["), take_until("]"), tag("]")).parse(input)?;
    Ok((
        input,
        Node {
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
fn parse_edge(input: &str) -> IResult<&str, Edge> {
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
        Edge {
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
    for line in input.lines() {
        match parse_line(line) {
            Ok((input, result)) => match result {
                ParsedLine::Node(node) => nodes.push(node),
                ParsedLine::Edge(edge) => edges.push(edge),
                ParsedLine::Blank => (),
                ParsedLine::Comment(_) => (),
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

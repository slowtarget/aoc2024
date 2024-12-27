use std::fmt::Write as _;
use std::io::Write as _;
use std::fs::File;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, space1},
    combinator::{map, opt},
    multi::separated_list1
    ,
    IResult,
};
use std::time::Instant;
use timing_util::measure_time;
#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: Option<u8>,
}

#[derive(Debug, Clone)]
enum GateType {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct Gate{
    gate_type: GateType,
    inputs: [usize; 2],
    output: usize,
}

#[derive(Debug)]
struct Circuit {
    wires: Vec<Wire>,
    gates: Vec<Gate>,
}

// Parse a single wire (e.g., x00: 1 or y03 without a value)
fn parse_wire(input: &str) -> IResult<&str, Wire> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric())(input)?;
    let (input, _) = opt(tag(": "))(input)?;
    let (input, value) = opt( digit1)(input)?;
    // println!("Wire: {:#?}", (name, value));
    let value = value.and_then(|v| v.parse::<u8>().ok()); // Parse safely, return None if parsing fails
    Ok((input, Wire { name: name.to_string(), value }))
}
// Parse a gate type (AND, OR, XOR)
fn parse_gate_type(input: &str) -> IResult<&str, GateType> {
    alt((
        map(tag("AND"), |_| GateType::AND),
        map(tag("OR"), |_| GateType::OR),
        map(tag("XOR"), |_| GateType::XOR),
    ))(input)
}
// Parse a single gate (e.g., `x00 OR x03 -> fst`)
fn parse_gate(input: &str) -> IResult<&str, (GateType, [Wire;2], Wire)> {
    let (input, in1)  = parse_wire(input)?;
    let (input, _) = space1(input)?;
    let (input, gate_type) = parse_gate_type(input)?;
    let (input, _) = space1(input)?;
    let (input, in2)  = parse_wire(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space1(input)?;
    let (input, output) = parse_wire(input)?;
    // println!("Gate: {:#?}",  (gate_type.clone(), [in1.clone(), in2.clone()], output.clone()));


    Ok((
        input,
        (
            gate_type,
            [in1, in2],
            output
        )
    ))
}

// Parse the full input into wires and gates
fn parse(input: &str) -> IResult<&str, (Vec<Wire>, Vec<(GateType, [Wire;2], Wire)>)> {
    let (input, wires) = separated_list1(tag("\n"), parse_wire)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, gates) = separated_list1(tag("\n"), parse_gate)(input)?;
    Ok((input, (wires, gates)))
}

fn part_1(wires: &Vec<Wire>, gates: &Vec<Gate>) -> String {
    let mut wires = wires.clone();
    let mut changes = true;
    while changes {
        changes = false;
        for gate in gates {
            if let Some(_output) = wires[gate.output].value {
                continue;
            }
            if let (Some(in1), Some(in2)) = (wires[gate.inputs[0]].value, wires[gate.inputs[1]].value) {
                let result = match gate.gate_type {
                    GateType::AND => in1 & in2,
                    GateType::OR => in1 | in2,
                    GateType::XOR => in1 ^ in2,
                };
                wires[gate.output].value = Some(result);
                changes = true;
            }
        }
    }
    let mut zs: Vec<(&String, u8)> = wires
        .iter()
        .filter(|wire| wire.name.chars().nth(0) == Some('z'))
        .map(|wire| (&wire.name,wire.value.unwrap()))
        .collect::<Vec<(&String, u8)>>();
    zs.sort();
    zs.reverse();
    zs.iter().for_each(|(name, value)| {
        println!("{}: {}", name, value);
    });
    let binary = zs.iter().map(|(_name, value)| value).map(|v| v.to_string()).collect::<Vec<String>>().join("");
    let decimal = u64::from_str_radix(&binary, 2).unwrap();
    decimal.to_string()
}
fn generate_plantuml(wires: &[Wire], gates: &[Gate]) -> String {
    let mut plantuml = String::new();
    writeln!(plantuml, "@startuml").unwrap();
    writeln!(plantuml, "left to right direction").unwrap();
    writeln!(plantuml, "").unwrap();
    
    writeln!(plantuml).unwrap();
    // Define inputs and outputs
    let mut names = wires.iter().map(|wire| wire.name.clone()).collect::<Vec<String>>();
    names.sort();
    for (i, wire) in names.iter().enumerate() {
        if let Some(wire_label) = wire_rectangle(wire) {
            let color = match wire_label {
                "in" => "#green",
                "out" => "#red",
                _ => "#black",
            };
            writeln!(plantuml, "storage {} \"{}\" as {}", color, wire_label, wire).unwrap();
        }
    }
    writeln!(plantuml).unwrap();
    
    // Define gates and connections
    for (i, gate) in gates.iter().enumerate() {
        let gate_label = match gate.gate_type {
            GateType::AND => "AND",
            GateType::OR => "OR",
            GateType::XOR => "XOR",
        };
        let shape = match gate.gate_type {
            GateType::AND => "component",
            GateType::OR => "cloud",
            GateType::XOR => "boundary",
        };
        writeln!(plantuml, "{} \"{}\" as gate_{}", shape, gate_label, i).unwrap();
    }


    for (i, gate) in gates.iter().enumerate() {
        let output = &wires[gate.output];

        for input_wire_idx in gate.inputs {
            let input_wire = &wires[input_wire_idx];
            if let Some(_wire_label) = wire_rectangle(input_wire.name.as_str()) {
                writeln!(plantuml, "{} --> gate_{}:{}", input_wire.name, i, input_wire.name).unwrap();
            } else {
                for (j, other_gate) in gates.iter().enumerate() {
                    if other_gate.output == input_wire_idx {
                        writeln!(plantuml, "gate_{} --> gate_{}:{}", j, i, input_wire.name).unwrap();
                    }
                }
            }
        }

        if output.name.starts_with('z') {
            writeln!(plantuml, "gate_{} --> {}:{}", i, output.name, output.name).unwrap();
        }
    }
    writeln!(plantuml).unwrap();

    writeln!(plantuml, "@enduml").unwrap();
    plantuml
}

fn wire_rectangle(wire: &str) -> Option<&str> {
    let wire_label = match wire.chars().nth(0) {
        Some('x') => Some("in"),
        Some('y') => Some("in"),
        Some('z') => Some("out"),
        _ => None,
    };
    wire_label
}
fn write_string_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
fn part_2(wires: &Vec<Wire>, gates: &Vec<Gate>) -> String {
    write_string_to_file("input/day24.backup.puml", &generate_plantuml(wires, gates)).unwrap();
    String::new()
}

pub(crate) fn solve<'a>(input: String) -> (String, String) {
    match parse(&input) {
        Ok((_remaining, (wires, gates))) => {

            // println!("Wires: {:?}", wires.len());
            // println!("Wires: {:?}", wires);
            // println!("Gates: {:?}", gates.len());
            // println!("Wires: {:?}", wires.len());
            // println!("Gates: {:?}", gates);
            let (wires, gates) = prep(wires, gates);
            let part_1_result: String = measure_time!(part_1(&wires, &gates));
            let part_2_result: String = measure_time!(part_2(&wires, &gates));
            (part_1_result.to_string(), part_2_result)
        }
        Err(err) => {
            eprintln!("Error parsing input: {err}");
            (String::new(), String::new())
        }
    }
}

fn prep<'a>(
    wires: Vec<Wire>,
    gates: Vec<(GateType, [Wire; 2], Wire)>,
) -> (Vec<Wire>, Vec<Gate>) {
    // Collect all wires into a single vector
    let mut updated_wires = wires.clone();
    gates.iter().for_each(|(_gate_type, [in1, in2], output)| {
        [in1.clone(), in2.clone(), output.clone()].iter().for_each(|w| {
            if !updated_wires.iter().any(|wire| wire.name == w.name) {
                updated_wires.push(w.clone());
            }
        });
    });

    // Create gates with references to the updated wires
    let mut updated_gates: Vec<Gate> = Vec::new();
    for (gate_type,  [in1, in2], output) in gates {
        let in1 = updated_wires.iter().position(|wire| wire.name == in1.name).unwrap();
        let in2 = updated_wires.iter().position(|wire| wire.name == in2.name).unwrap();
        let output = updated_wires.iter().position(|wire| wire.name == output.name).unwrap();
        updated_gates.push(Gate {
            gate_type: gate_type.clone(),
            inputs: [in1, in2],
            output,
        });
    }

    ( updated_wires,
         updated_gates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_provided() {
        let input = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02".to_string();
        let (_input, (wires, gates)) = parse(&input).unwrap();
        let ( wires, gates) = prep(wires, gates);
        assert_eq!(part_1(&wires, &gates), "4".to_string());
    }
    #[test]
    fn test_part_1_larger_provided() {
        let input = input();

        let (_input, (wires, gates)) = parse(&input).unwrap();
        let ( wires, gates) = prep(wires, gates);

        assert_eq!(part_1(&wires, &gates), "2024".to_string());
    }
    
    fn input() -> String {
        "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj".to_string()
    }
    #[test]
    fn prep_test() {
        let wires = vec![
            Wire { name: "x00".to_string(), value: Some(1) },
            Wire { name: "x01".to_string(), value: Some(0) },
            Wire { name: "x02".to_string(), value: Some(1) },
            Wire { name: "x03".to_string(), value: Some(1) },
            Wire { name: "x04".to_string(), value: Some(0) },
            Wire { name: "y00".to_string(), value: Some(1) },
            Wire { name: "y01".to_string(), value: Some(1) },
            Wire { name: "y02".to_string(), value: Some(1) },
            Wire { name: "y03".to_string(), value: Some(1) },
            Wire { name: "y04".to_string(), value: Some(1) }
        ];
        let gates = vec![
            (GateType::XOR,
                 [Wire { name: "x00".to_string(), value: None }, Wire { name: "x01".to_string(), value: None }],
             Wire { name: "z01".to_string(), value: None },),
            
        ];
        let (wires, gates) = prep(wires, gates);
        assert_eq!(wires.len(), 11);
        assert_eq!(gates.len(), 1);
        assert_eq!(wires[gates[0].inputs[0]].name, "x00");
        assert_eq!(wires[gates[0].inputs[0]].value, Some(1));
        assert_eq!(wires[gates[0].inputs[1]].name, "x01");
        assert_eq!(wires[gates[0].output].name, "z01");
        
        
    }
}






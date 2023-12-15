use std::collections::HashMap;
use crate::part1::hash;

struct Lens {
    pub label: String,
    pub focal_length: u32,
}
impl Lens {
    pub fn new(label: String, focal_length: u32) -> Self {
        Self { label, focal_length }
    }
}
impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let data_split : Vec<&str> = input.split(",").collect();

    let mut map: HashMap<u32, Vec<Lens>> = HashMap::new();

    for &data in data_split.iter() {

        // get label, operation, focal length
        let pos_operation = data.find(|x| x == '=' || x == '-').unwrap();
        let label: &str = &data[..pos_operation];
        let operation: &str = &data[pos_operation..pos_operation + 1];
        let mut focal_len: &str = "";

        if operation.contains('=') {
            focal_len = &data[pos_operation + 1..pos_operation + 2];
        }

        let hash = hash(label);

        // find bin with hash, or make default, as if empty nothing to remove
        let lenses = map.entry(hash).or_default();

        match operation {
            "-" => {

                // remove lens with label by checking if it exists at any position, then remove it
                if let Some(index) = lenses.iter().position(|x| x.label == label) {
                    lenses.remove(index);
                }
            },
            "=" => {
                // make new lens
                let new_lense = Lens::new(label.to_string(), focal_len.parse::<u32>().unwrap());
                // 2 cases, if lense exist in box, then replace it with new lense
                // otherwise if no lens in box with label, we push the new lens to the back of the box
                match lenses.iter().position(|x| x.label == label) {
                    Some(index) => {
                        lenses[index] = new_lense;
                    },
                    None => {
                        lenses.push(new_lense)
                    }
                }
            }
            _ => {},
        }
    }

    // calculate combined focusing power for install validation
    let mut sum: u32 = 0;
    for (box_id, lens_box) in map.iter() {
        for (i, lens) in lens_box.iter().enumerate() {
            sum += (box_id+1) * (i as u32+1) * lens.focal_length;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
                assert_eq!("145", process(input)?);
                Ok(())
    }
}
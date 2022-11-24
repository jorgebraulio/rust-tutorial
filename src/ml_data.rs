use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MLData {
    pub nodes: Vec<Node>,
    pub tree: Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub i: String,
    #[serde(default = "default_fnz_id")]
    fnz_id: String,
    pub a: HashMap<String, String>,
}

fn default_fnz_id() -> String {
    String::from("-1")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeNode {
    pub i: String,
    pub c: Option<Vec<TreeNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MLDataContainer {
    element_statistics: MLData,
}

fn read_ml_json(path: &Path) -> MLDataContainer{

    let json_str = fs::read_to_string(path).unwrap();

    let mut deserializer = serde_json::Deserializer::from_str(&json_str);
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    //deserializer.disable_recursion_limit();
    MLDataContainer::deserialize(deserializer).unwrap()
}

fn calc_val(v1: f32, v2:f32) -> Option<f32>{
    if v2 == 0.0{
        None
    }else {
        Some(v1/v2)
    }
}

fn sum_rate(v1: f32, v2:f32, val: f32) -> Option<f32>{
    let rate = calc_val(v1, v2)?;
    //match rate {
    //    Some(r) =>{
    //        Some(r + val)
    //    },
    //    None => {
    //        None
    //    }
    //}

    Some(rate + val)
}

/****************EMPIEZA TAREA ****************************/


fn find_xx(nodos: &Vec<Node>)->Node{
    //Inicializamos el nodo que regresaremos clonando cualquier elemento de la lista original
    let mut nodexx:Node=nodos[1].clone();
    //For sobre los nodos
    for it in nodos.iter(){
        //Si tiene campo XX clonamos a nuestro nodo 
        if it.a.contains_key("XX"){nodexx=it.clone()};
    }
    //Regresamos el nodo con xx
    nodexx
}

pub fn load_json_test(){
    let path = Path::new("resources/mldata.json");
    let data = read_ml_json(&path);

    println!("{}", data.element_statistics.nodes.len());
    println!("{:?}",find_xx(&data.element_statistics.nodes).a);
    println!("{:?}",correlacion(find_xx(&data.element_statistics.nodes),&data.element_statistics.nodes));
}
fn correlacion(nodoxx:Node, nodos:&Vec<Node>) -> Vec<f64> {
    //Sumamos cuantos elementos relevantes hay
    let n_relevantes = nodoxx.a.iter().filter(|(k,v)| *k != "WH" &&  *k != "XX" && *k != "LT" && *k != "HT" && *k != "TP").count();
    //mapeo, a cada elemento del vector de nodos le asignara un valor f64, que se regresa en la ultima linea
    nodos.iter().map(|g|{
        //Contador
        let mut sum =0;
        //For sobre las llaves del nodoxx
        for (k,v) in nodoxx.a.iter(){
            //Se puede poner estrellita como en C, con filter hacemos count, estamos iterando sobre los campos del nodo actual
            sum += g.a.iter().filter(|(gk,gv)| *gk==k && *gv == v &&  *gk != "WH" &&  *gk != "XX" && *gk != "LT" && *gk != "HT" && *gk != "TP").count();
        }
        (sum as f64)/(n_relevantes as f64) //normalizamos la suma
        //Se rgresa sum por que es la ultima instruccion en el contexto de las llaves de map |g|{}
    }).collect()
}
/****************  TERMINA TAREA ****************************/
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    height :f32
}

#[cfg(test)]
mod test{
    use std::path::Path;
    use crate::ml_data::{Person, read_ml_json};

    #[test]
    fn json_test(){
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let p: Person = serde_json::from_str(data).unwrap();

        // Do things just like with any other Rust data structure.
        println!("Please call {} at the number {}", p.name, p.phones[0]);
    }

    #[test]
    fn find_xx(nodos: &Vec<Node>)->Node{
        let mut iterator = nodos.iter().cycle();
        let mut nodexx:Node;
        for it in nodos.iter(){
            iterator.by_ref().take(it).for_each(|x| if x.a.contains_key("XX"){nodexx=x.clone()});
        }
        nodexx
    }
    #[test]
    fn load_json_test(){
        let path = Path::new("resources/mldata.json");
        let data = read_ml_json(&path);

        println!("{}", data.element_statistics.nodes.len());
        println!("{:?}",find_xx(data.elemente_statistics.nodes).a);
    }

}
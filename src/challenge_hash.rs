//ajouter ces bibliotheques externes au projet
pub mod HashCashMod {
    use md5::Digest;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::num::ParseIntError;
    use std::str::FromStr;
    use serde::Deserializer;
    use serde_json::Value;

//mod message;
//use message::Message;


    // Ajout des bibliothèques externes nécessaires
    extern crate rand;
    extern crate md5;

    enum Challenge {
        MD5HashCash
    }

    // Structure qui représente le résultat au challenge donnée
    struct ChallengeResult {
        answer: Answer,
        next_target: String
    }

    impl ChallengeResult {
        fn new(answer: Answer, next_target: String) -> Self {
            ChallengeResult {
                answer,
                next_target
            }
        }
    }

    // Structure qui représente la réponse au challenge donnée
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Answer {
        challenge_name: String,
        challenge_answer: MD5HashCashOutput
    }

    impl Answer {
        fn new(challenge_name: String, challenge_answer: MD5HashCashOutput) -> Self {
            Answer {
                challenge_name,
                challenge_answer
            }
        }
    }

    // Structure qui représente les données en entrée du challenge
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MD5HashCashInput {
        // complexité en bits
        complexity: u32,
        // message à signer
        message: String,
    }

    // Structure qui représente les données en sortie du challenge
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MD5HashCashOutput {
        // graine utilisée pour résoudre le challenge
        seed: u64,
        // hashcode trouvé en utilisant la graine + le message
        hashcode: String,
    }

    // Implémentation de la fonction Hash pour la structure MD5HashCashInput
    impl Hash for MD5HashCashInput {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.complexity.hash(state);
            self.message.hash(state);
        }
    }

    // Implémentation de la fonction FromStr pour la structure MD5HashCashInput
    impl FromStr for MD5HashCashInput {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.splitn(2, " ");
            let complexity = parts.next().ok_or("Missing complexity".to_string())?;
            let message = parts.next().ok_or("Missing message".to_string())?;

            Ok(MD5HashCashInput {
                complexity: complexity.parse().unwrap(),
                message: message.to_string(),
            })
        }
    }

    // converti le caractère hexadécimal en binaire
    fn format_binary(c: char) -> String {
        if c == '0' {
            return "0000".to_string();
        } else if c == '1' {
            return "0001".to_string();
        } else if c == '2' {
            return "0010".to_string();
        } else if c == '3' {
            return "0011".to_string();
        } else if c == '4' {
            return "0100".to_string();
        } else if c == '5' {
            return "0101".to_string();
        } else if c == '6' {
            return "0110".to_string();
        } else if c == '7' {
            return "0111".to_string();
        } else if c == '8' {
            return "1000".to_string();
        } else if c == '9' {
            return "1001".to_string();
        } else if c == 'A' {
            return "1010".to_string();
        } else if c == 'B' {
            return "1011".to_string();
        } else if c == 'C' {
            return "1100".to_string();
        } else if c == 'D' {
            return "1101".to_string();
        } else if c == 'E' {
            return "1110".to_string();
        } else if c == 'F' {
            return "1111".to_string();
        };
        return "1".to_string();
    }

    // converti un hexadécimal en binaire
    fn convert_hex_to_binary(hex: &String) -> String {
        let mut binary = "".to_string();
        for i in hex.chars() {
            binary = binary.to_owned() + &format_binary(i);
        }
        binary
    }

    fn verify_bit_zero(number: u32, binary: String) -> bool {
        for i in 0..number {
            if binary.chars().nth(i as usize) != Some('0') {
                return false;
            }
        }
        return true;
    }

    // Fonction principale qui résout le challenge
    fn solve_md5_hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput {
// Map qui stocke les résultats déjà calculés pour éviter de refaire les calculs
        let mut cache: HashMap<MD5HashCashInput, MD5HashCashOutput> = HashMap::new();

// Génère une valeur de graine aléatoire
        let mut seed: u64 = rand::random();

// Tant qu'on n'a pas trouvé une valeur de graine qui résout le challenge
        while !cache.contains_key(&input) {
            let mut seed_binary: String = format!("{:X}", seed);
            /*for i in seed.to_le_bytes(){
            seed_binary = seed_binary + &*i.to_string();
        }*/
            // Calcul du hashcode en utilisant la graine + le message
            let mut binary = md5::Md5::new();
            binary.update(seed_binary.to_owned() + &input.message.to_string());
            let binary_bin = binary.finalize();
            //let bin = format!("{:X}", binary_bin);
            //println!("{bin}");
            let hashcode = format!("{:X}", binary_bin);
            //println!("{hashcode}");
            //let hashcode = format!("{:X}", md5::compute(format!("{:X}{}", seed, input.message)));

            // Convertit le hashcode en binaire
            let hashcode_bin = convert_hex_to_binary(&hashcode);
            //let hashcode_bin = format!("{:b}", hashcode.into_bytes());

            //regarder si les complexity bits sont egaux à 0 convertir hashcode en bits
            let hashcode_clone = hashcode.clone();
            // Vérifie si le hashcode comprend au moins "complexity" bits égaux à 0
            if verify_bit_zero(input.complexity, hashcode_bin) {
                //Stocke le résultat dans la map pour éviter de refaire les calculs
                println!("{}", seed_binary);
                cache.insert(input.clone(), MD5HashCashOutput { seed, hashcode: hashcode_clone });
            }

            // Génère une nouvelle valeur de graine aléatoire
            seed = rand::random();
        }

// Retourne le résultat du challenge
        cache[&input].clone()
    }

    // Implémentation du trait Challenge pour la structure MD5HashCash
    pub struct MD5HashCashChallenge {
        md5HashCashInput: MD5HashCashInput,
        md5HashCashOuput: MD5HashCashOutput,
    }

    impl MD5HashCashChallenge {
        // Le nom du challenge est "MD5 Hash Cash"
        pub fn name() -> String {
            "MD5 Hash Cash".to_string()
        }

        // Crée un nouveau challenge à partir des données en entrée
        pub fn new(hashInput: MD5HashCashInput) -> Self {
            let hashInput_clone = hashInput.clone();
            MD5HashCashChallenge {
                md5HashCashInput: hashInput,
                md5HashCashOuput: solve_md5_hash_cash(hashInput_clone),
            }
        }

        // Résout le challenge
        pub fn solve(&self) -> MD5HashCashOutput {
            self.md5HashCashOuput.clone()
        }

        // Vérifie qu'une sortie est valide pour le challenge
        pub fn verify(&self, output: MD5HashCashOutput) -> bool {
            // Vérifie si les données en sortie sont égales à celles calculées lors de la création du challenge
            self.md5HashCashOuput == output
        }
    }

    /*fn message_to_json(structure: Message)-> String{
    serde_json::to_string(&structure).ok().unwrap()
}

fn json_to_message(json: String)->Message{
    serde_json::from_str(&json.as_str()).unwrap()
}*/

    fn main() {
        /*println!("{:?}", message::Message::ChallengeTimeout {message: "coucou ceci est un tesr".to_string()});
    let data = r#"
        {"EndOfGame":{"leader_board":{"publicLeaderBoard":[{"name":"free_patato","stream_id":"127.0.0.1","score":10,"steps":20,"is_active":true,"total_used_time":1.234},{"name":"dark_salad","stream_id":"127.0.0.1","score":6,"steps":200,"is_active":true,"total_used_time":0.1234}]}}}"#;
    println!("json {}", message_to_json(message::Message::ChallengeTimeout {message: "coucou ceci est un tesr".to_string()}));

    // Parse the string of data into serde_json::Value.
    //let v: Value = json_to_message(data.to_string());
    //let m = message::Message::Welcome {version: v["Welcome"]["version"].as_u64().unwrap() as u8};
    let message = json_to_message(data.to_string());
    // Access parts of the data by indexing with square brackets.
    //println!("struct rust {:?}", m);
     println!("struct rust {:?}", message);*/


        /*let data = r#"{
    "Welcome":{
        "version":1
        }
    }"#;
    let srcMsg = Message::Welcome.version = 2;*/
        //let v: Value = serde_json::from_str(data).unwrap();
        //let message_to_json = serde(srcMsg);

        //println!("{}", message_to_json);
        // Parse les données en entrée du challenge
        /*let input: MD5HashCashInput = "9 hello".parse().unwrap();

    // Crée un nouveau challenge
    let challenge = MD5HashCash::new(input);

    // Résout le challenge
    let output = challenge.solve();

    // Vérifie si la sortie est valide pour le challenge
    let result = challenge.verify(output.clone());
    let output_clone = output.clone();
    // Affiche le résultat
    println!("Result = {}", result);
    println!("Resultat hashcode = {}", output.clone().hashcode);
    let mut seed_binary="".to_string();
    for i in output_clone.seed.to_le_bytes(){
      seed_binary = seed_binary.to_string() + &i.to_string();
    }
    println!("Resultat seed = {}", seed_binary);
    let challenge_answer = Answer::new("MD5".to_string(), output_clone);
    let challenge_result = ChallengeResult::new(challenge_answer.clone(), "next_target".to_string());
    println!("{}", challenge_answer.challenge_name.clone());
    println!("{}", challenge_result.next_target.clone());
    let input = MD5HashCashInput{complexity: 9, message: String::from("hello")};
    let output = solve_md5_hash_cash(input);
    println!("Resultat hashcode = {}", output.hashcode);
    /*let mut seed_binary="".to_string();
    for i in output.seed.to_le_bytes(){
        seed_binary = seed_binary.to_string() + &i.to_string();
    }*/
    println!("Resultat seed = {}", output.seed);*/
        //println!("Bit code = {}", format!("{:b}", output.seed))
    }


    /*
 * INFO COMPLÉMENTAIRE
Vous pouvez ensuite appeler la méthode solve_md5_hash_cash en lui passant les données en entrée du challenge, et elle retournera les données en sortie du challenge. Par exemple :

let input: MD5HashCashInput = "9 hello".parse().unwrap();
let output = solve_md5_hash_cash(input);*/
}
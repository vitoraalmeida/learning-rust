use std::fs::read_to_string;
use regex::Regex;
use clap::{App, Arg};


fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let haystack = read_to_string(input).unwrap();
    
    //numero de linhas de contexto acima e abaixo do padrão
    let context_lines = 2; 
    //armazena os numeros das linhas onde o padrão foi encontrado
    let mut tags: Vec<usize> = Vec::new();
    //armazena o contexto (linhas acima, a linha do padrão, e linhas abaixo)
    //na forma (numero da linha, linha)
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    for (i, line) in haystack.lines().enumerate() {
        match re.find(line) {
            Some(_) => {
                tags.push(i);
                let v = Vec::with_capacity(2*context_lines + 1);
                ctx.push(v);
            },
            None => (),
        }
    }

    if tags.len() == 0 {
        return;
    }

    //para cada linha, verifica se ela faz parte do contexto de alguma das linhas
    //encontradas como contendo o padrão e insere essa linha e seu conteudo no
    //vector de contexto.
    for (i, line) in haystack.lines().enumerate() {
        for (j,tag) in tags.iter().enumerate() {
            // faz a subtração de de valores sem sinal retornando 0 se for
            //abaixo de zero. usize não pode ser negativo.
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

}

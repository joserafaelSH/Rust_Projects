use std::env;
mod genetic;
mod ler_arquivo_entrada;

//cargo run nomearquivo tamanhodaentrada tampopulacao

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let tam = args[2].parse::<usize>().unwrap();
    let pop_size = args[3].parse::<usize>().unwrap();
    let mut cities = vec![[0.0; 2]; tam];
    ler_arquivo_entrada::ler_entrada(filename, &mut cities);
    println!("Tamanho da entrada: {}", tam);

    // deve ser mut tirei pro compilador nao encher o saco
    let population = genetic::criar_populacao_inicial(tam, pop_size);
    let fitness = genetic::calcular_fitness(population, tam, &mut cities);
    println!("{:?}", fitness);
    //selecao por roleta
    //cruzamento, algum que funcione de preferencia sem utilizar coisas aleatorias
    //mutacao
    //remover o pior e adicionar o melhor
}

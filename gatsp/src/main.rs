use rand::Rng;
use std::env;
mod genetic;
mod ler_arquivo_entrada;

//cargo run nomearquivo tamanhodaentrada tampopulacao taxa_mutacao geracoes
//cargo run att48.txt 48 10 0.2 5

fn main() {
    let args: Vec<String> = env::args().collect();
    //nome do arquivo de entrada
    let filename = &args[1];
    //tamanho da entrada
    let tam = args[2].parse::<usize>().unwrap();
    //tamanho da populacao
    let pop_size = args[3].parse::<usize>().unwrap();
    //taxa de mutacao
    let mutacao = args[4].parse::<f64>().unwrap();
    //geracoes
    let max_generations = args[5].parse::<usize>().unwrap();

    //lendo informacoes da entrada, posicoes de cada cidade
    let mut cities = vec![[0.0; 2]; tam];
    ler_arquivo_entrada::ler_entrada(filename, &mut cities);

    let mut contador: usize = 0;

    //criando populacao inicial
    let mut population = genetic::criar_populacao_inicial(tam, pop_size);

    while contador < max_generations {
        //calculando o fitness de cada cromossomo
        let fitness = genetic::calcular_fitness(population.clone(), tam, &mut cities);

        //idx do melhor cromossomo

        let idx_melhor = genetic::melhor_cromossomo(fitness.clone());

        let aux = genetic::value_of_cromossome(population[idx_melhor].clone(), tam, &mut cities);
        println!("{}", aux);
        // if aux <= 33000.0 {
        //     break;
        // }
        //primeiro individuo selecionado
        let idx1: i32 = genetic::selecao_por_roleta(fitness.clone());
        //segundo individuo selecionado
        let mut idx2: i32 = genetic::selecao_por_roleta(fitness.clone());
        while idx1 == idx2 {
            idx2 = genetic::selecao_por_roleta(fitness.clone());
        }

        //melhor resultado do cruzamento entre os dois individuos selecionados
        let mut filho: Vec<i32> = genetic::cruzamento_cx(
            population[idx1 as usize].clone(),
            population[idx2 as usize].clone(),
            &mut cities,
        );
        //idx do pior cromossomo
        let idx_pior = genetic::pior_cromossomo(fitness.clone());

        //numero aleatorio para definir se vai ou nao acontecer a mutacao
        let mut rng = rand::thread_rng();
        let rand_choice = rng.gen_range(0.0..1.0);
        if rand_choice <= mutacao {
            genetic::mutacao(&mut filho);
        }

        //trocando o pior pelo novo individuo gerado

        population[idx_pior] = filho;

        contador += 1;
    }
}

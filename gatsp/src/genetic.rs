
use rand::seq::SliceRandom;
use rand::Rng;

pub fn distance(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let x = bx - ax;
    let y = by - ay;
    let aux = (x * x) + (y * y);
    return aux.sqrt(); 
    
}

pub fn value_of_cromossome(cromossome: Vec<i32>, tam: usize, cities: &mut Vec<[f64; 2]>) -> f64 {
    let mut c: usize = 0;
    let mut values: f64 = 0.0;

    while c < tam - 1 {
        values += distance(
            cities[cromossome[c] as usize][0],
            cities[cromossome[c] as usize][1],
            cities[cromossome[c + 1] as usize][0],
            cities[cromossome[c + 1] as usize][1],
        );
        c += 1;
    }
    return values;
}

pub fn criar_populacao_inicial(tam: usize, pop_size: usize) -> Vec<Vec<i32>> {
    let mut base: Vec<i32> = Vec::new();
    let mut c: i32 = 0;

    while (c as usize) < tam {
        base.push(c);
        c += 1;
    }
    c = 0;
    let mut rng = rand::thread_rng();

    let mut population: Vec<Vec<i32>> = Vec::new();
    while (c as usize) < pop_size {
        let mut aux = base.clone();
        aux.shuffle(&mut rng);

        population.push(aux);
        c += 1;
    }

    return population;
}

pub fn calcular_fitness(
    cromossomes: Vec<Vec<i32>>,
    tam: usize,
    cities: &mut Vec<[f64; 2]>,
) -> Vec<f64> {
    let mut fitness: Vec<f64> = Vec::new();
    for i in cromossomes {
        fitness.push(1.0 / value_of_cromossome(i, tam, cities));
    }
    return fitness;
}

pub fn selecao_por_roleta(fitness: Vec<f64>) -> i32 {
    let sum_fitness: f64 = fitness.iter().sum();
    let mut prob: Vec<f64> = Vec::new();
    for i in fitness {
        prob.push(i / sum_fitness);
    }

    let mut offset: f64 = 0.0;
    let mut pick: i32 = 0;

    let mut c: i32 = 0;

    let mut rng = rand::thread_rng();
    let rand_choice = rng.gen_range(0.0..1.0);
    while (c as usize) < prob.len() {
        offset += prob[c as usize];
        if rand_choice < offset {
            pick = c;
            break;
        }
        c += 1;
    }
    return pick;
}

pub fn cruzamento_cx(pai1: Vec<i32>, pai2: Vec<i32>, cities: &mut Vec<[f64; 2]>) -> Vec<i32> {
    let mut filho1: Vec<i32>;
    let mut filho2: Vec<i32>;

    filho1 = pai2.clone();
    let mut c: usize = 0;
    filho1[c as usize] = pai1[c as usize];
    c = pai2.iter().position(|&r| r == filho1[c]).unwrap();
    while (c as usize) != 0 {
        filho1[c as usize] = pai1[c as usize];
        c = pai2.iter().position(|&r| r == filho1[c]).unwrap();
    }

    filho2 = pai1.clone();
    c = 0;
    filho2[c as usize] = pai2[c as usize];
    c = pai1.iter().position(|&r| r == filho2[c]).unwrap();
    while (c as usize) != 0 {
        filho2[c as usize] = pai2[c as usize];
        c = pai1.iter().position(|&r| r == filho2[c]).unwrap();
    }

    if value_of_cromossome(filho1.clone(), filho1.len(), cities)
        < value_of_cromossome(filho2.clone(), filho2.len(), cities)
    {
        return filho2;
    } else {
        return filho1;
    }
}

pub fn pior_cromossomo(fitness: Vec<f64>) -> usize {
    let mut minvalues: f64 = fitness[0];
    let mut minidx: usize = 0;
    let mut c = 0;
    while c < fitness.len() {
        if minvalues > fitness[c] {
            minvalues = fitness[c];
            minidx = c;
        }
        c += 1;
    }

    return minidx;
}


pub fn melhor_cromossomo(fitness: Vec<f64>) -> usize {
    let mut maxvalues: f64 = fitness[0];
    let mut maxidx: usize = 0;
    let mut c = 0;
    while c < fitness.len() {
        if maxvalues < fitness[c] {
            maxvalues = fitness[c];
            maxidx = c;
        }
        c += 1;
    }

    return maxidx;
}

pub fn mutacao(alvo: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    let rand_choice1 = rng.gen_range(0..alvo.len());
    let mut rand_choice2 = rng.gen_range(0..alvo.len());
    while rand_choice1 == rand_choice2 {
        rand_choice2 = rng.gen_range(0..alvo.len());
    }

    (alvo[rand_choice1], alvo[rand_choice2]) = (alvo[rand_choice2], alvo[rand_choice1]);
}

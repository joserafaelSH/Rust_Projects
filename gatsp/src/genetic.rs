use rand::seq::SliceRandom;

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

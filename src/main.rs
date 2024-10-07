use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant; // Importar la biblioteca para medir el tiempo

fn sum_chunk(chunk: &[i32]) -> i32 {
    chunk.iter().sum()
}

fn fork_join_sum(data: &[i32], num_threads: usize) -> i32 {
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let mut handles = Vec::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, data.len());

        // Clonamos el Arc para moverlo a la closure
        let results_clone = Arc::clone(&results);
        let chunk = data[start..end].to_vec(); // Convertimos a Vec<i32>

        // Creación del hilo
        let handle = thread::spawn(move || {
            let sum = sum_chunk(&chunk); // Pasamos la referencia de la nueva Vec
            // Almacenamos el resultado
            results_clone.lock().unwrap().push(sum);
        });

        handles.push(handle);
    }

    // Esperar que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    // Sumar los resultados parciales
    let total_sum: i32 = results.lock().unwrap().iter().sum();
    total_sum
}

fn main() {
    let data = (1..=10_000).collect::<Vec<i32>>();
    let num_threads = 4;

    // Iniciar el temporizador
    let start_time = Instant::now();

    let total = fork_join_sum(&data, num_threads);

    // Calcular la duración
    let duration = start_time.elapsed();

    // Mostrar el resultado y el tiempo en milisegundos
    println!("La suma total es: {}", total);
    println!("Tiempo de ejecución: {} nanosegundos", duration.as_nanos());
}

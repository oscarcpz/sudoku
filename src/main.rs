const SUM_SUDOKU: i8 = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1;

fn main() {
    let mut sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
        //[5, 3, 4, 6, 7, 8, 9, 1, 2],
        [5, 3, 0, 6, 7, 8, 0, 1, 2],
        //[6, 7, 2, 1, 9, 5, 3, 4, 8],
        [0, 7, 2, 0, 9, 5, 0, 4, 0],
        //[1, 9, 8, 3, 4, 2, 5, 6, 7],
        [1, 0, 0, 0, 4, 2, 5, 6, 7],
        //[8, 5, 9, 7, 6, 1, 4, 2, 3],
        [8, 5, 9, 0, 0, 0, 0, 2, 3],
        //[4, 2, 6, 8, 5, 3, 7, 9, 1],
        [4, 2, 6, 0, 0, 0, 7, 9, 1],
        //[7, 1, 3, 9, 2, 4, 8, 5, 6],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        //[9, 6, 1, 5, 3, 7, 2, 8, 4],
        [9, 6, 1, 0, 3, 7, 2, 8, 4],
        //[2, 8, 7, 4, 1, 9, 6, 3, 5],
        [2, 8, 7, 4, 1, 9, 0, 0, 0],
        //[3, 4, 5, 2, 8, 6, 1, 7, 9]
        [3, 4, 5, 2, 8, 0, 0, 7, 9]
    ];
    print_sudoku(&mut sudoku);
    println!("Resuelve sudoku");
    let solve: bool = resolve_sudoku(&mut sudoku, false);
    println!("Sudoku resuelto: {}", solve);
    print_sudoku(&mut sudoku);
}

fn print_sudoku(sudoku: &mut [[i8; 9]]) {
    for i in 0..9 {
        println!("{:?}", sudoku[i]);
    }
}

fn resolve_sudoku(sudoku: &mut [[i8; 9]], solved: bool) -> bool {

    println!("Resolviendo");

    if solved {
        return true; //punto de finalizacion de backtracking
    }

    let mut has_been_solved: bool = false;
    for i in 0..9 { //lo recorro en diagonal para no tratar de resolver 9 veces una misma columna o fila
        if sudoku[i][i] == 0 {
            if !resolve_row(i, sudoku) { //intentamos resolver fila
                if !resolve_col(i, sudoku) { //intentamos resolver columna
                    let row = get_corner_from_square_giving_cell(i);
                    let col = get_corner_from_square_giving_cell(i);
                    has_been_solved = resolve_square(row, col, sudoku); //intentamos resolver cuadrado
                }
            }

        }
    }

    if !has_been_solved {
        //comienza el backtracking
        //1. buscamos el primer 0 en el sudoku
        for i in 0..9 {
            for j in 0..9 {
                if sudoku[i][j] == 0 {
                    //array de numeros posibles [1..9]
                    //por cada numero posible
                    for num in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        if is_number_in_row(i, num, sudoku) {
                            continue;
                        }
                        if is_number_in_col(j, num, sudoku){
                            continue;
                        }
                        let row = get_corner_from_square_giving_cell(i);
                        let col = get_corner_from_square_giving_cell(j);
                        if is_number_in_square(row, col, num, sudoku) {
                            continue;
                        }
                        //si estamos en este punto, el numero no se ha usado todavia
                        //2. colocamos el numero en el sudoku
                        sudoku[i][j] = num;
                        //3. tratamos de resolver
                        if !resolve_sudoku(sudoku, false) {
                            //ponemos de nuevo un cero en ese punto y nos vamos al siguiente numero
                            sudoku[i][j] = 0;
                        }
                    }                    
                }
            }
        }
    }

    true
}

//fn resolve_row(row: &mut [i8; 9]) -> bool { //debe recibir una referencia a un array mutable
fn resolve_row(row: usize, sudoku: &mut [[i8; 9]]) -> bool {
    //primero contamos los 0
    let mut sum: i8 = 0;
    let mut zero_counter: i8 = 0;
    for i in 0..9 {
        let pointer = &mut sudoku[row][i]; //no es necesario crear este puntero, se puede hacer con una variable normal. Es simplemente didactico
        if *pointer == 0 { //para acceder al valor de una referencia solo se puede hacer usando el puntero (*)
            zero_counter += 1;
        }
        sum += *pointer;
    }

    if zero_counter > 1 {
        return false;
    }

    if zero_counter == 0 {
        return true;
    }

    //tenemos solo una casilla por resolver, resolvemos
    for i in 0..9 {
        let pointer = &mut sudoku[row][i];
        if *pointer == 0 {
            *pointer = SUM_SUDOKU - sum;
            break;
        }
    }

    true
}

fn resolve_col(col: usize, sudoku: &mut [[i8; 9]]) -> bool {

    //primero contamos los 0
    let mut sum: i8 = 0;
    let mut zero_counter: i8 = 0;
    for i in 0..9 {
        let pointer = &mut sudoku[i][col]; //no es necesario crear este puntero, se puede hacer con una variable normal. Es simplemente didactico
        if *pointer == 0 { //para acceder al valor de una referencia solo se puede hacer usando el puntero (*)
            zero_counter += 1;
        }
        sum += *pointer;
    }

    if zero_counter > 1 {
        return false;
    }

    if zero_counter == 0 {
        return true;
    }

    //tenemos solo una casilla por resolver, resolvemos
    for i in 0..9 {
        let pointer = &mut sudoku[i][col];
        if *pointer == 0 {
            *pointer = SUM_SUDOKU - sum;
            break;
        }
    }

    true
}

fn resolve_square(row: usize, col: usize, sudoku: &mut [[i8; 9]]) -> bool {

    //aseguramos que las posiciones recibidas son correctas para no tener un Out Of Bound
    if row > 5 {
        return false;
    }

    if col > 5 {
        return false;
    }

    //primero contamos los 0
    let mut sum: i8 = 0;
    let mut zero_counter: i8 = 0;
    for i in row..(row+3) {
        for j in col..(col+3) {
            let num = sudoku[i][j];
            if num == 0 {
                zero_counter += 1;
            }
            sum += num;
        }

        if zero_counter > 1 {
            return false;
        }

        if zero_counter == 0 {
            return true;
        }
    }

    //tenemos solo una casilla por resolver
    for i in row..(row+3) {
        for j in col..(col+3) {
            //en este punto sí necesitamos hacer uso de punteros porque necesitamos llegar a una casilla de la matriz
            let pointer = &mut sudoku[i][j];
            if *pointer == 0 {
                *pointer = SUM_SUDOKU - sum;
                break;
            }
        }
    }

    true

}

//dado un una casilla cualquiera del cuadrante, devulve la esquina superior izquierda de dicho cuadrante
//es util para otras funciones en las que necesitamos trabajar con cuadrantes
fn get_corner_from_square_giving_cell(cell: usize) -> usize {
    if cell < 3 {
        return 0;
    }
    if cell < 6 {
        return 3;
    }

    return 6;
}

fn is_number_in_row(row: usize, num: i8, sudoku: & [[i8; 9]]) -> bool {
    let mut found = false;
    for i in 0..9 {
        let pointer = & sudoku[row][i];
        if *pointer == num {
            found = true;
            break;
        }
    }

    found
}

fn is_number_in_col(col: usize, num: i8, sudoku: & [[i8; 9]]) -> bool {
    let mut found = false;
    for i in 0..9 {
        let pointer = & sudoku[i][col];
        if *pointer == num {
            found = true;
            break;
        }
    }

    found
}

fn is_number_in_square(row: usize, col: usize, num: i8, sudoku: & [[i8; 9]]) -> bool {
    let mut found = false;
    for i in row..(row+3) {
        for j in col..(col+3) {
            //en este punto sí necesitamos hacer uso de punteros porque necesitamos llegar a una casilla de la matriz
            let pointer = & sudoku[i][j];
            if *pointer == num {
                found = true;
                break;
            }
        }
    }

    found
}


//TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sudoku_by_row() {
        let mut sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 0, 1, 4, 2, 3], // 0 -> 6
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 0, 5, 2, 8, 0, 1, 7, 9] // 4, 6
        ];

        assert!(resolve_row(3, &mut sudoku)); //se envia la referencia indicando que es mutable
        assert_eq!(sudoku[3], [8, 5, 9, 7, 6, 1, 4, 2, 3]);
        assert!(resolve_row(4, &mut sudoku)); //comprueba que todas las casillas tienen un numero
        assert_eq!(resolve_row(8, &mut sudoku), false); //comprueba que si la fila tiene mas de un cero no se puede resolver
    }

    #[test]
    fn check_sudoku_by_col() {
        let mut sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 0, 1, 4, 2, 3], // 0 -> 6
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 0, 1, 5, 3, 7, 2, 8, 4], // 0 -> 6
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 0, 5, 2, 8, 0, 1, 7, 9] // 4, 6
        ];

        assert!(resolve_col(4, &mut sudoku)); //se envia la referencia indicando que es mutable
        assert!(resolve_col(0, &mut sudoku)); //comprueba que todas las casillas tienen un numero
        assert_eq!(resolve_col(1, &mut sudoku), false); //comprueba que si la fila tiene mas de un cero no se puede resolver
        assert_eq!(sudoku[3][4], 6);
    }

    #[test]
    fn check_sudoku_by_square() {
        let mut sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 0, 1, 4, 2, 3], // 0 -> 6
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 0, 1, 5, 3, 7, 2, 8, 4], // 0 -> 6
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 0, 5, 2, 8, 0, 1, 7, 9] // 4, 6
        ];

        assert!(resolve_square(3, 3, &mut sudoku)); //se envia la referencia indicando que es mutable
        assert_eq!(sudoku[3][4], 6);
        assert!(resolve_square(0, 0, &mut sudoku)); //comprueba que todas las casillas tienen un numero
        assert_eq!(resolve_square(6, 0, &mut sudoku), false); //comprueba que si la fila tiene mas de un cero no se puede resolver
        assert!(resolve_square(3, 3, &mut sudoku)); //se envia la referencia indicando que es mutable
        assert_eq!(sudoku[3][4], 6);
    }

    #[test]
    fn check_is_number_in_row() {
        let sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]
        ];
        assert!(is_number_in_row(2, 7, &sudoku));
        assert_eq!(is_number_in_row(2, 10, &sudoku), false);
    }

    #[test]
    fn check_is_number_in_col() {
        let sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]
        ];
        assert!(is_number_in_col(2, 7, &sudoku));
        assert_eq!(is_number_in_col(2, 10, &sudoku), false);
    }

    #[test]
    fn check_is_number_in_square() {
        let sudoku: [[i8; 9]; 9] = [ //sudoku debe ser mutable
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]
        ];
        assert!(is_number_in_square(0, 0, 7, &sudoku));
        assert_eq!(is_number_in_square(6, 6, 10, &sudoku), false);
    }

    #[test]
    fn check_get_corner() {
        assert_eq!(get_corner_from_square_giving_cell(2), 0);
        assert_eq!(get_corner_from_square_giving_cell(4), 3);
        assert_eq!(get_corner_from_square_giving_cell(7), 6);
        assert_eq!(get_corner_from_square_giving_cell(6), 6);
    }

}

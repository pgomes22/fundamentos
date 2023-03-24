#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta, 
    Quinta,
    Sexta,
    Sabado
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RbgColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn main() {
    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    matriz();

    let dia: DiaDaSemana = DiaDaSemana::Sabado;

    println!("Eh fim de semana? {}", eh_fim_de_semana(dia));

    println!("A cor escolhida é: {}", what_is_color(Color::RbgColor(255, 255, 255)));
    println!("A cor escolhida é: {}", what_is_color(Color::CymkColor{cyan: 100, magenta: 50, yellow: 70, black: 255 }));

    conteudo_opcional();
    vectors();
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

fn what_is_color(cor: Color) -> String {
    match cor {
        Color::Red => String::from("vermelho"),
        Color::Green => String::from("verde"),
        Color::Blue => String::from("azul"),
        Color::RbgColor(0, 0, 0) | Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } 
            => String::from("preta"),
        Color::RbgColor(_, _, _) => String::from("RBG desconhecido!"),
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => String::from("CYMK desconhecido!")
    }
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));
    
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };    

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza que o arquivo tem dados: {}", valor);
    }
}

#[allow(unused)]
fn ler_arquivo(caminho_arquivo: String) -> Option<String> {    
    Some(String::from("Conteudo do arquivo..."))
}

fn vectors() {
    // let mut notas: Vec<f32> = Vec::new();
    let mut notas: Vec<f32> = vec![10.0, 8.0, 6.5];
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    println!("Nota 6 = {}", match notas.get(6) {
        Some(n) => *n,
        None => 0.0
    })

}
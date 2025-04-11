/// Represents the possible states of an animal.

struct Animal{
    name : String,
    state : AnimalState,
}

enum AnimalState {
    Extant,          // The animal is extant (still exists).
    Extinct(u16),         // The animal is extinct (no longer exists).
    PossiblyExtinct(u16, String) // The animal is possibly extinct (its status is unclear, but it may be extinct).
}

/// Prints the state of the given animal.
///
/// # Arguments
///
/// * `state` - The current state of the animal, which can be `Extant`, `Extinct`, or `PossiblyExtinct`.
/// 

impl Animal {
    fn to_string(&self) -> String {
        format!("Nome: {} \nEstado: {}", self.name, self.state.to_string())
    }
}

impl AnimalState{
    fn to_string(&self) -> String{
         match self{
            AnimalState::Extant => {
                "É extante".to_string()
            },
            AnimalState::Extinct(ano) => format!("Foi extinto em {}", ano),
            AnimalState::PossiblyExtinct(ano ,comentario ) => format!("Foi possivelmente extinto em {}, {}", ano, comentario)

        }
    }
}

fn main() {
    let a1 = Animal{
        name: "Cavalo".to_string(),
        state: AnimalState::Extant,
    };

    let a2 = Animal{
        name: "Dodo".to_string(),
        state: AnimalState::Extinct(1600),
    };

    let a3 = Animal{
        name: "Arara-azul".to_string(),
        state: AnimalState::PossiblyExtinct((1600), ("Visto em goias".to_string()))
    };

    println!("{}", a3.to_string())

}

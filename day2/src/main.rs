#[derive(PartialEq, Debug, Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors
}

impl Throw {
    fn win(&self) -> Self {
        match self {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper
        }
    }

    fn lose(&self) -> Self {
        match self {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        }
    }
}

impl PartialOrd for Throw {
    fn lt(&self, other: &Self) -> bool {
        match self {
            Throw::Rock => {
                matches!(other, Throw::Paper)
            },
            Throw::Paper => {
                matches!(other, Throw::Scissors)
            },
            Throw::Scissors => {
                matches!(other, Throw::Rock)
            }
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Throw::Rock => {
                matches!(other, Throw::Scissors)
            },
            Throw::Paper => {
                matches!(other, Throw::Rock)
            },
            Throw::Scissors => {
                matches!(other, Throw::Paper)
            }
        }
    }

    fn le(&self, _other: &Self) -> bool {
        false
    }

    fn ge(&self, _other: &Self) -> bool {
        false
    }

    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }

}

impl From<&str> for Throw {
    fn from(x: &str) -> Self {
        match x {
            "A" => Throw::Rock,
            "B" => Throw::Paper,
            "C" => Throw::Scissors,
            "X" => Throw::Rock,
            "Y" => Throw::Paper,
            "Z" => Throw::Scissors,
            _ => panic!("Unknown input")
        }
    }
}

enum Strategy {
    Win,
    Lose,
    Draw
}

impl From<&str> for Strategy {
    fn from(x: &str) -> Self {
        match x {
            "X" => Strategy::Lose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => panic!("Unknown input: {x}")
        }
    }
}

fn main() -> color_eyre::Result<()> {
    let inp = include_str!("../data/inp1.txt");
    let mut score = 0;
    for line in inp.lines() {
        let (opponent, me) = line.split_once(' ').unwrap();
        let opp_throw = Throw::from(opponent);
        // let my_throw = Throws::from(me);
        let strategy = Strategy::from(me);


        let my_throw = match strategy {
            Strategy::Win => opp_throw.lose(),
            Strategy::Lose => opp_throw.win(),
            Strategy::Draw => opp_throw.clone(),
        };

        let my_throw_score = match my_throw {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3
        };
        score += my_throw_score;

        if my_throw == opp_throw {
            println!("{opp_throw:?} == {my_throw:?}!");
            score += 3;
        }
        else if my_throw > opp_throw {
            println!("My {my_throw:?} beats opponent's {opp_throw:?}!");
            score += 6;
        }
        else if my_throw < opp_throw {
            println!("My {my_throw:?} loses to opponent's {opp_throw:?}!");
        }
        else {
            println!("Unknown: Mine-{my_throw:?} - Opp-{opp_throw:?}")
        }

    }
    println!("Score: {score}");


    Ok(())
}

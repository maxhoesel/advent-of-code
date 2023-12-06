use day05::Almanac;

use anyhow::anyhow;
use anyhow::Result;
use day05::SeedRangeAlmanac;

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

#[tokio::main]
async fn main() -> Result<()> {
    let test_almanac = Almanac::parse(TEST)?;
    for seed in test_almanac.seeds() {
        println!(
            "Seed {} should be planted in {}",
            seed,
            test_almanac.seed_location(*seed)
        );
    }

    let input_almanac = Almanac::parse(INPUT)?;
    let input_min = input_almanac
        .seeds()
        .map(|seed| input_almanac.seed_location(*seed))
        .min()
        .ok_or(anyhow!("No seeds in input"))?;
    println!("The minimum location value is: {}", input_min);

    let test_almanac_ranged = SeedRangeAlmanac::parse(TEST)?;
    let test_ranged_minseed = test_almanac_ranged.lowest_seed();
    let test_ranged_minloc = test_almanac_ranged.seed_location(test_ranged_minseed);
    println!(
        "The minimum test seed is {} for a location of {}",
        test_ranged_minseed, test_ranged_minloc
    );

    let input_almanac_ranged = SeedRangeAlmanac::parse(INPUT)?;
    let input_almanac_minseed = input_almanac_ranged.lowest_seed();
    let input_almanac_minloc = input_almanac_ranged.seed_location(input_almanac_minseed);
    println!(
        "The minimum input seed is {} for a location of {}",
        input_almanac_minseed, input_almanac_minloc
    );

    Ok(())
}

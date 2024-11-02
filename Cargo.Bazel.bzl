load("@rules_rust//crate_universe:defs.bzl", "crate")

crates = {
    # Testing
    # "arbitrary": crate.spec(
    #     version = "1",
    #     features = ["derive"],
    # },
    "quickcheck": crate.spec(
        version = "1",
    ),
    "quickcheck_macros": crate.spec(
        version = "1",
    ),

    # Generates random numbers
    # "rand": crate.spec(
    #     version = "0.8.5",
    # ),
}
